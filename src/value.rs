use napi::{bindgen_prelude::*, JsUnknown};
use netidx::protocol::value::Value;

#[napi(js_name = "ValueType")]
pub enum JsValueType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    Duration,
    DateTime,
    Buffer,
    Result,
    Array,
}

#[napi(js_name = "Value")]
pub struct JsValue(Value);

impl From<Value> for JsValue {
    fn from(value: Value) -> Self {
        JsValue(value)
    }
}

#[napi]
impl JsValue {
    #[napi(js_name = "type")]
    pub fn kind(&self) -> JsValueType {
        match &self.0 {
            Value::U32(_)
            | Value::V32(_)
            | Value::I32(_)
            | Value::Z32(_)
            | Value::U64(_)
            | Value::V64(_)
            | Value::I64(_)
            | Value::Z64(_) => JsValueType::Integer,
            Value::F32(_) | Value::F64(_) => JsValueType::Float,
            Value::DateTime(_) => JsValueType::DateTime,
            Value::Duration(_) => JsValueType::Duration,
            Value::String(_) => JsValueType::String,
            Value::Bytes(_) => JsValueType::Buffer,
            Value::Ok | Value::Error(_) => JsValueType::Result,
            Value::Array(_) => JsValueType::Array,
            Value::True | Value::False => JsValueType::Boolean,
            Value::Null => JsValueType::Null,
        }
    }

    // CR estokes: build a javascript object from the value
    #[napi]
    pub fn get(&self, env: Env) -> Result<JsUnknown> {
        match &self.0 {
            Value::U32(u) | Value::V32(u) => Ok(env.create_uint32(*u)?.into_unknown()),
            Value::I32(i) | Value::Z32(i) => Ok(env.create_int32(*i)?.into_unknown()),
            Value::U64(u) | Value::V64(u) => {
                Ok(env.create_int64(*u as i64)?.into_unknown())
            }
            Value::I64(i) | Value::Z64(i) => Ok(env.create_int64(*i)?.into_unknown()),
            Value::F32(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
            Value::F64(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
            Value::String(s) => Ok(env.create_string(s)?.into_unknown()),
        }
    }

    #[napi]
    pub fn get_int(&self) -> Result<i64> {
        self.0.clone().cast_to::<i64>().map_err(|_| {
            let m = format!("Can't cast {} to a number", self.0);
            Error::new(Status::NumberExpected, m)
        })
    }

    #[napi]
    pub fn get_float(&self) -> Result<f64> {
        self.0.clone().cast_to::<f64>().map_err(|_| {
            let m = format!("Can't cast {} to a float", self.0);
            Error::new(Status::NumberExpected, m)
        })
    }

    //...
}
