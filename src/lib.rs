#![allow(dead_code)]

use netidx::{
    config::Config,
    publisher::{BindCfg, Publisher},
    subscriber::DesiredAuth,
};

#[macro_use]
extern crate napi_derive;

mod path;
mod util;
mod value;

#[napi(js_name = "Publisher")]
struct JsPublisher {
    inner: Publisher,
}

#[napi]
impl JsPublisher {
    pub async fn new() -> Option<JsPublisher> {
        let cfg = Config::load_default().unwrap();

        Some(Self {
            inner: Publisher::new(cfg, DesiredAuth::Anonymous, BindCfg::Local)
                .await
                .unwrap(),
        })
    }
}
