use std::time;

use napi::{
    bindgen_prelude::{BigInt, Buffer, Either16, Error, Null, Result, Status},
    JsDate,
};
use netidx::{
    config::Config,
    path::Path,
    publisher::{BindCfg, Publisher as NPublisher, Val as NVal},
    subscriber::{DesiredAuth, Value},
};

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct U32 {
    pub inner: u32,
}

impl Into<Value> for U32 {
    fn into(self) -> Value {
        Value::U32(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct V32 {
    pub inner: u32,
}

impl Into<Value> for V32 {
    fn into(self) -> Value {
        Value::V32(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct I32 {
    pub inner: i32,
}

impl Into<Value> for I32 {
    fn into(self) -> Value {
        Value::I32(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct Z32 {
    pub inner: i32,
}

impl Into<Value> for Z32 {
    fn into(self) -> Value {
        Value::Z32(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone)]
pub struct U64 {
    pub inner: BigInt,
}

impl Into<Value> for U64 {
    fn into(self) -> Value {
        Value::U64(self.inner.get_u64().1)
    }
}

#[napi(constructor)]
#[derive(Clone)]
pub struct V64 {
    pub inner: BigInt,
}

impl Into<Value> for V64 {
    fn into(self) -> Value {
        Value::V64(self.inner.get_u64().1)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct I64 {
    pub inner: i64,
}

impl Into<Value> for I64 {
    fn into(self) -> Value {
        Value::I64(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct Z64 {
    pub inner: i64,
}

impl Into<Value> for Z64 {
    fn into(self) -> Value {
        Value::Z64(self.inner)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct F32 {
    pub inner: f64,
}

impl Into<Value> for F32 {
    fn into(self) -> Value {
        Value::F32(self.inner as f32)
    }
}

#[napi(constructor)]
#[derive(Clone, Copy)]
pub struct F64 {
    pub inner: f64,
}

impl Into<Value> for F64 {
    fn into(self) -> Value {
        Value::F64(self.inner)
    }
}

#[napi]
#[derive(Clone, Copy)]
pub struct Duration(time::Duration);

#[napi]
impl Duration {
    #[napi(constructor)]
    pub fn new(seconds: BigInt, nanoseconds: u32) -> Self {
        Duration(time::Duration::new(seconds.get_u64().1, nanoseconds))
    }
}

impl Into<Value> for Duration {
    fn into(self) -> Value {
        Value::Duration(self.0)
    }
}

fn to_value(
    v: Either16<
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
) -> Value {
    match v {
        Either16::A(_) => Value::Null,
        Either16::B(v) => v.into(),
        Either16::C(v) => v.into(),
        Either16::D(v) => v.into(),
        Either16::E(v) => v.into(),
        Either16::F(v) => v.into(),
        Either16::G(v) => v.into(),
        Either16::H(v) => Value::U64(v.inner.get_u64().1),
        Either16::I(v) => Value::V64(v.inner.get_u64().1),
        Either16::J(v) => v.into(),
        Either16::K(v) => v.into(),
        Either16::L(v) => v.into(),
        Either16::M(v) => v.into(),
        Either16::N(v) => v.into(),
        Either16::O(_v) => todo!(),
        Either16::P(_v) => todo!(),
    }
}

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
        NPublisher::new(cfg, DesiredAuth::Anonymous, BindCfg::Local).await.unwrap(),
    ))
}
