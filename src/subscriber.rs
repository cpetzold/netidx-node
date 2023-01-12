use crate::util::js_of_value;
use futures::{channel::mpsc, prelude::*, select_biased};
use fxhash::{FxHashMap, FxHashSet};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{
        ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    },
};
use netidx::{
    config::Config,
    pool::Pooled,
    subscriber::{
        DesiredAuth, Dval as NDVal, Event, SubId, Subscriber as NSubscriber,
        UpdatesFlags, Value,
    },
};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};
use tokio::task;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CbId(u64);

impl CbId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        CbId(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

type NCb = ThreadsafeFunction<Value, ErrorStrategy::Fatal>;

enum DvToBe {
    RegisterCb(SubId, CbId, NCb),
    UnregisterCb(SubId, CbId),
}

#[napi]
pub struct DVal {
    callbacks: FxHashSet<CbId>,
    subscriber: Arc<SubscriberInner>,
    val: NDVal,
}

impl Drop for DVal {
    fn drop(&mut self) {
        let id = self.val.id();
        for cbid in &self.callbacks {
            let _: std::result::Result<_, _> =
                self.subscriber.to_be.unbounded_send(DvToBe::UnregisterCb(id, *cbid));
        }
    }
}

#[napi]
impl DVal {
    #[napi(
        ts_args_type = "callback: (value: null | boolean | string | number | Buffer | Array<any>) => void"
    )]
    pub fn on_update(&mut self, callback: JsFunction) {
        let tsfn: NCb = callback
            .create_threadsafe_function(0, |mut ctx| {
                Ok(vec![js_of_value(&mut ctx.env, &ctx.value)?])
            })
            .unwrap();
        let cbid = CbId::new();
        let id = self.val.id();
        let _: std::result::Result<_, _> =
            self.subscriber.to_be.unbounded_send(DvToBe::RegisterCb(id, cbid, tsfn));
        self.callbacks.insert(cbid);
    }
}

struct SubscriberInner {
    subscriber: NSubscriber,
    to_be: mpsc::UnboundedSender<DvToBe>,
    updates: mpsc::Sender<Pooled<Vec<(SubId, Event)>>>,
}

#[napi]
pub struct Subscriber(Arc<SubscriberInner>);

#[napi]
impl Subscriber {
    #[napi]
    pub fn subscribe(&self, path: String) -> DVal {
        let val = self.0.subscriber.durable_subscribe(path.into());
        val.updates(UpdatesFlags::empty(), self.0.updates.clone());
        DVal { subscriber: Arc::clone(&self.0), val, callbacks: HashSet::default() }
    }
}

async fn subscriber_task(
    mut from_js: mpsc::UnboundedReceiver<DvToBe>,
    mut from_sub: mpsc::Receiver<Pooled<Vec<(SubId, Event)>>>,
) {
    let mut callbacks: FxHashMap<SubId, FxHashMap<CbId, NCb>> = HashMap::default();
    loop {
        select_biased! {
            mut batch = from_sub.select_next_some() => {
                for (id, ev) in batch.drain(..) {
                    if let Event::Update(v) = ev {
                        if let Some(cbs) = callbacks.get(&id) {
                            for cb in cbs.values() {
                                cb.call(v.clone(), ThreadsafeFunctionCallMode::Blocking);
                            }
                        }
                    }
                }
            },
            req = from_js.next() => match req {
                None => break,
                Some(DvToBe::RegisterCb(id, cbid, cb)) => {
                    callbacks.entry(id).or_insert_with(HashMap::default).insert(cbid, cb);
                }
                Some(DvToBe::UnregisterCb(id, cbid)) => {
                    if let Entry::Occupied(mut e) = callbacks.entry(id) {
                        let h = e.get_mut();
                        h.remove(&cbid);
                        if h.is_empty() {
                            e.remove();
                        }
                    }
                }
            },
            complete => break,
        }
    }
}

#[napi]
pub async fn create_subscriber() -> Option<Subscriber> {
    let cfg = Config::load_default().ok()?;
    let (to_be_tx, to_be_rx) = mpsc::unbounded();
    let (updates_tx, updates_rx) = mpsc::channel(3);
    task::spawn(subscriber_task(to_be_rx, updates_rx));
    Some(Subscriber(Arc::new(SubscriberInner {
        subscriber: NSubscriber::new(cfg, DesiredAuth::Local).ok()?,
        to_be: to_be_tx,
        updates: updates_tx,
    })))
}
