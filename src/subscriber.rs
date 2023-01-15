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
type NCb = ThreadsafeFunction<BatchedUpdate, ErrorStrategy::CalleeHandled>;

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
        let val = self.subscriber.durable_subscribe(path);
        val.updates(UpdatesFlags::empty(), self.updates.clone());

        let id = val.id().inner();
        self.dvals.insert(id, val.clone());
        id
    }

    #[napi]
    pub fn unsubscribe(&mut self, sub_id: BigInt) {
        let (_, id, _) = sub_id.get_u64();
        self.dvals.remove(&id);
    }
}

async fn subscriber_task(mut from_sub: mpsc::Receiver<BatchedUpdate>, callback: NCb) {
    while let Some(batch) = from_sub.next().await {
        callback.call(Ok(batch), ThreadsafeFunctionCallMode::Blocking);
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

#[napi(
    ts_args_type = "callback: (batch: Event[]) => void, configJson?: string, authJson?: string"
)]
pub fn create_subscriber(
    callback: JsFunction,
    config_json: Option<String>,
    auth_json: Option<String>,
) -> Result<Subscriber> {
    let tsfn: NCb = callback.create_threadsafe_function(
        0,
        move |mut ctx: ThreadSafeCallContext<BatchedUpdate>| {
            Ok(vec![ctx
                .value
                .drain(..)
                .map(|(id, event)| match event {
                    NEvent::Unsubscribed => Event {
                        id: id.inner().into(),
                        kind: EventType::Unsubscribed,
                        value: None,
                    },
                    NEvent::Update(v) => Event {
                        id: id.inner().into(),
                        kind: EventType::Update,
                        value: js_of_value(&mut ctx.env, &v.clone()).ok(),
                    },
                })
                .collect::<Vec<Event>>()])
        },
    )?;

    let config = if let Some(config_json) = config_json {
        Config::parse(&config_json)
    } else {
        Config::load_default()
    }
    .map_err(|_| Error::from_reason("Couldn't load config"))?;

    let auth: DesiredAuth = if let Some(auth_json) = auth_json {
        serde_json::from_str(auth_json.as_str())
            .map_err(|_| Error::from_reason("Couldn't load desired auth"))?
    } else {
        config.default_auth()
    };

    let (updates_tx, updates_rx) = mpsc::channel(3);
    task::spawn(subscriber_task(updates_rx, tsfn));
    Ok(Subscriber {
        subscriber: NSubscriber::new(config, auth)
            .map_err(|_| Error::from_reason("Couldn't create subscriber"))?,
        updates: updates_tx,
        dvals: FxHashMap::default(),
    })
}
