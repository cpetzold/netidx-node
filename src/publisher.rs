use napi::{bindgen_prelude::*, JsDate};
use netidx::{
    config::Config,
    path::Path,
    publisher::{BindCfg, Publisher as NPublisher, Val as NVal},
    subscriber::DesiredAuth,
};

use crate::value::*;

#[napi]
pub struct Val(NVal);

#[napi]
pub struct Publisher(NPublisher);

#[napi]
impl Publisher {
    #[napi]
    pub fn publish(
        &self,
        path: String,
        value: Either16<
            Null,
            bool,
            String,
            &U32,
            &V32,
            &I32,
            &Z32,
            &U64,
            &V64,
            &I64,
            &Z64,
            &F32,
            &F64,
            &Duration,
            JsDate,
            Buffer,
        >,
    ) -> Result<Val> {
        self.0
            .publish(Path::from(path), to_value(value))
            .map_err(|_| Error::from_status(Status::GenericFailure))
            .map(|v| Val(v))
    }
}

#[napi]
pub async fn create_publisher() -> Option<Publisher> {
    let cfg = Config::load_default().unwrap();

    Some(Publisher(
        NPublisher::new(cfg, DesiredAuth::Local, BindCfg::Local).await.unwrap(),
    ))
}
