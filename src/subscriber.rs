use std::sync::Arc;

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
    dvals: Arc<FxHashMap<SubId, (Path, Dval)>>,
    path_to_subid: FxHashMap<Path, SubId>,
}

#[napi]
impl Subscriber {
    #[napi]
    pub fn subscribe(&mut self, path: String) {
        let p: Path = Path::from(path.clone());
        let val = self.subscriber.durable_subscribe(p.clone());
        val.updates(UpdatesFlags::empty(), self.updates.clone());
        Arc::make_mut(&mut self.dvals).insert(val.id(), (p.clone(), val.clone()));
        self.path_to_subid.insert(p, val.id());
    }

    #[napi]
    pub fn unsubscribe(&mut self, path: String) {
        let p: Path = Path::from(path.clone());
        if let Some(id) = self.path_to_subid.get(&p) {
            Arc::make_mut(&mut self.dvals).remove(id);
            self.path_to_subid.remove(&p);
        }
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
    pub path: Option<String>,
    pub value: Option<JsUnknown>,
    pub kind: EventType,
}

#[napi]
pub fn create_subscriber(callback: JsFunction) -> Result<Subscriber> {
    let dvals: Arc<FxHashMap<SubId, (Path, Dval)>> = Arc::new(FxHashMap::default());
    let idvals = dvals.clone();
    let tsfn: NCb = callback.create_threadsafe_function(
        0,
        move |mut ctx: ThreadSafeCallContext<BatchedUpdate>| {
            Ok(vec![ctx
                .value
                .drain(..)
                .map(|(id, event)| {
                    let path = idvals.get(&id).map(|(path, _)| path.clone().to_string());
                    Ok(match event {
                        NEvent::Unsubscribed => {
                            Event { path, kind: EventType::Unsubscribed, value: None }
                        }
                        NEvent::Update(v) => Event {
                            path,
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
        dvals: dvals.clone(),
        path_to_subid: FxHashMap::default(),
    })
}
