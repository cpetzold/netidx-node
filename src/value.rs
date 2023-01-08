use napi::bindgen_prelude::*;
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
