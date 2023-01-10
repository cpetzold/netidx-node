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

type ValueFromJsInner<'a> = Either16<
    Null,
    bool,
    String,
    &'a U32,
    &'a V32,
    &'a I32,
    &'a Z32,
    &'a U64,
    &'a V64,
    &'a I64,
    &'a Z64,
    &'a F32,
    &'a F64,
    &'a Duration,
    JsDate,
    Buffer,
>;

struct ValueFromJs<'a>(ValueFromJsInner<'a>);

impl Into<Value> for ValueFromJs<'_> {
    fn into(self) -> Value {
        match self.0 {
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
}

impl From<Value> for ValueFromJs<'a> {
    fn from(value: Value) -> Self {
        Self(match value {
            Value::U32(v) => ValueFromJsInner::D(&'a U32 { inner: v }),
            Value::V32(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::Z32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::V64(_) => todo!(),
            Value::I64(_) => todo!(),
            Value::Z64(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::DateTime(_) => todo!(),
            Value::Duration(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Bytes(_) => todo!(),
            Value::True => todo!(),
            Value::False => todo!(),
            Value::Null => todo!(),
            Value::Ok => todo!(),
            Value::Error(_) => todo!(),
            Value::Array(_) => todo!(),
        })
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
        let init: Value = ValueFromJs(value).into();
        self.0
            .publish(Path::from(path), init)
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
