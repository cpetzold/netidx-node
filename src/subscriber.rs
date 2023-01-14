use crate::util::js_of_value;
use futures::{channel::mpsc, prelude::*};
use fxhash::FxHashMap;
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{
        ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction,
        ThreadsafeFunctionCallMode,
    },
    JsUnknown,
};
use netidx::{
    config::Config,
    path::Path,
    pool::Pooled,
    subscriber::{
        DesiredAuth, Dval, Event as NEvent, SubId, Subscriber as NSubscriber,
        UpdatesFlags,
    },
};
use tokio::task;

type BatchedUpdate = Pooled<Vec<(SubId, NEvent)>>;
type NCb = ThreadsafeFunction<BatchedUpdate, ErrorStrategy::Fatal>;

#[napi]
pub struct Subscriber {
    subscriber: NSubscriber,
    updates: mpsc::Sender<BatchedUpdate>,
    dvals: FxHashMap<u64, Dval>,
}

#[napi]
impl Subscriber {
    #[napi]
    pub fn subscribe(&mut self, path: String) -> u64 {
        let path = Path::from(path);
        let val = self.subscriber.durable_subscribe(path.clone());
        val.updates(UpdatesFlags::empty(), self.updates.clone());

        let id = val.id().inner();
        self.dvals.insert(id, val.clone());
        val.id().inner()
    }

    #[napi]
    pub fn unsubscribe(&mut self, sub_id: BigInt) {
        let (_, id, _) = sub_id.get_u64();
        self.dvals.remove(&id);
    }
}

async fn subscriber_task(mut from_sub: mpsc::Receiver<BatchedUpdate>, callback: NCb) {
    loop {
        if let Some(batch) = from_sub.next().await {
            callback.call(batch, ThreadsafeFunctionCallMode::Blocking);
        }
    }
}

#[napi]
pub enum EventType {
    Unsubscribed,
    Update,
}

#[napi(object)]
pub struct Event {
    pub id: BigInt,
    pub value: Option<JsUnknown>,
    #[napi(js_name = "type")]
    pub kind: EventType,
}

#[napi(ts_args_type = "callback: (batch: Event[]) => void")]
pub fn create_subscriber(callback: JsFunction) -> Result<Subscriber> {
    let tsfn: NCb = callback.create_threadsafe_function(
        0,
        move |mut ctx: ThreadSafeCallContext<BatchedUpdate>| {
            Ok(vec![ctx
                .value
                .drain(..)
                .map(|(id, event)| {
                    Ok(match event {
                        NEvent::Unsubscribed => Event {
                            id: id.inner().into(),
                            kind: EventType::Unsubscribed,
                            value: None,
                        },
                        NEvent::Update(v) => Event {
                            id: id.inner().into(),
                            kind: EventType::Update,
                            value: Some(js_of_value(&mut ctx.env, &v.clone())?),
                        },
                    })
                })
                .collect::<Vec<Result<Event>>>()])
        },
    )?;
    let cfg =
        Config::load_default().map_err(|_| Error::from_reason("Couldn't load config"))?;
    let (updates_tx, updates_rx) = mpsc::channel(3);
    task::spawn(subscriber_task(updates_rx, tsfn));
    Ok(Subscriber {
        subscriber: NSubscriber::new(cfg, DesiredAuth::Local)
            .map_err(|_| Error::from_reason("Couldn't create subscriber"))?,
        updates: updates_tx,
        dvals: FxHashMap::default(),
    })
}
