use crate::util::js_of_value;
use futures::{channel::mpsc, prelude::*, select};
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
    dvals: FxHashMap<String, Dval>,
}

#[napi]
impl Subscriber {
    #[napi]
    pub fn subscribe(&mut self, path: String) {
        let val = self.subscriber.durable_subscribe(path.clone().into());
        val.updates(UpdatesFlags::empty(), self.updates.clone());
        self.dvals.insert(path, val);
    }

    #[napi]
    pub fn unsubscribe(&mut self, path: String) {
        self.dvals.remove(&path);
    }
}

async fn subscriber_task(mut from_sub: mpsc::Receiver<BatchedUpdate>, callback: NCb) {
    loop {
        select! {
            batch = from_sub.select_next_some() => {
                callback.call(batch, ThreadsafeFunctionCallMode::Blocking);
            },
            complete => break,
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
    // pub id: JsNumber,
    pub kind: EventType,
    pub value: Option<JsUnknown>,
}

#[napi]
pub fn create_subscriber(callback: JsFunction) -> Result<Subscriber> {
    let tsfn: NCb = callback.create_threadsafe_function(
        0,
        |mut ctx: ThreadSafeCallContext<BatchedUpdate>| {
            Ok(vec![ctx
                .value
                .drain(..)
                .map(|(_id, event)| {
                    Ok(match event {
                        NEvent::Unsubscribed => {
                            Event { kind: EventType::Unsubscribed, value: None }
                        }
                        NEvent::Update(v) => Event {
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
