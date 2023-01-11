use futures::channel::mpsc;
use napi::{bindgen_prelude::*, JsDate};

use netidx::{
    config::Config,
    subscriber::{DesiredAuth, Dval as NDVal, Subscriber as NSubscriber, UpdatesFlags},
};

use crate::value::*;

#[napi]
pub struct DVal(NDVal);

#[napi]
impl DVal {
    #[napi]
    pub async fn on_update<T>(&self, callback: T)
    where
        T: Fn(
            Either16<
                Null,
                bool,
                String,
                U32,
                V32,
                I32,
                Z32,
                U64,
                V64,
                I64,
                Z64,
                F32,
                F64,
                Duration,
                JsDate,
                Buffer,
            >,
        ) -> Result<()>,
    {
        callback(Either16::C("Hello world".to_string()));

        // let (tx, mut rx) = mpsc::channel(10);
        // self.0.updates(UpdatesFlags::empty(), tx);
        // while let Some(mut batch) = rx.next().await {
        //     for (_, v) in batch.drain(..) {
        //         // callback(from_value(v));
        //     }
        // }
    }
}

#[napi]
pub struct Subscriber(NSubscriber);

#[napi]
impl Subscriber {
    #[napi]
    pub fn subscribe(&self, path: String) -> DVal {
        DVal(self.0.durable_subscribe(path.into()))
    }
}

#[napi]
pub async fn create_subscriber() -> Option<Subscriber> {
    let cfg = Config::load_default().unwrap();

    Some(Subscriber(NSubscriber::new(cfg, DesiredAuth::Local).unwrap()))
}
