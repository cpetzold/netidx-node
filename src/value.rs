use napi::{bindgen_prelude::*, JsUnknown};
use netidx::protocol::value::Value as NValue;

#[napi]
pub enum ValueType {
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

#[napi]
pub struct Value(NValue);

impl From<NValue> for Value {
    fn from(value: NValue) -> Self {
        Value(value)
    }
}

#[napi]
impl Value {
    #[napi(js_name = "type")]
    pub fn kind(&self) -> ValueType {
        match &self.0 {
            NValue::U32(_)
            | NValue::V32(_)
            | NValue::I32(_)
            | NValue::Z32(_)
            | NValue::U64(_)
            | NValue::V64(_)
            | NValue::I64(_)
            | NValue::Z64(_) => ValueType::Integer,
            NValue::F32(_) | NValue::F64(_) => ValueType::Float,
            NValue::DateTime(_) => ValueType::DateTime,
            NValue::Duration(_) => ValueType::Duration,
            NValue::String(_) => ValueType::String,
            NValue::Bytes(_) => ValueType::Buffer,
            NValue::Ok | NValue::Error(_) => ValueType::Result,
            NValue::Array(_) => ValueType::Array,
            NValue::True | NValue::False => ValueType::Boolean,
            NValue::Null => ValueType::Null,
        }
    }

    // CR estokes: build a javascript object from the value
    #[napi]
    pub fn get(&self, env: Env) -> Result<JsUnknown> {
        match &self.0 {
            NValue::U32(u) | NValue::V32(u) => Ok(env.create_uint32(*u)?.into_unknown()),
            NValue::I32(i) | NValue::Z32(i) => Ok(env.create_int32(*i)?.into_unknown()),
            NValue::U64(u) | NValue::V64(u) => {
                Ok(env.create_int64(*u as i64)?.into_unknown())
            }
            NValue::I64(i) | NValue::Z64(i) => Ok(env.create_int64(*i)?.into_unknown()),
            NValue::F32(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
            NValue::F64(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
            NValue::String(s) => Ok(env.create_string(s)?.into_unknown()),
            NValue::Bytes(b) => {
                // CR estokes: avoid the copy by wrapping the Bytes type?
                let mut buf = env.create_buffer(b.len())?;
                buf.copy_from_slice(&*b);
                Ok(buf.into_unknown())
            }
            _ => Ok(env.create_int32(42)?.into_unknown()),
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
