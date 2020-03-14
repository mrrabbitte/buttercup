use serde_json::{Number, Value};

use crate::arguments::ArgumentType;
use crate::arguments::values::ValueHolder;

pub struct Transformer;

impl Transformer {

    pub fn transform(value: &Value) -> Result<ValueHolder, String> {
        return match value {
            Value::Null => Result::Err(String::from("Got null value.")),
            Value::Bool(val) => Result::Ok(ValueHolder::Boolean(*val)),
            Value::Number(val) => Transformer::handle(val),
            Value::String(_) => Result::Err(String::from("Illegal value")),
            val => Result::Err(format!("Value not supported {:?}", val))
        };
    }

    fn handle(val: &Number) -> Result<ValueHolder, String> {
        if val.is_f64() {
            return Result::Ok(ValueHolder::Decimal(val.as_f64().unwrap()));
        }
        if val.is_i64() {
            let opt_i64 = val.as_i64();
            return match opt_i64 {
                Some(v) => Result::Ok(
                    ValueHolder::Integer(v)),
                None => Result::Err(String::from("Could not get i64 value."))
            };
        }
        if val.is_u64() {
            let opt_u64 = val.as_u64();
            return match opt_u64 {
                Some(v) => Result::Ok(
                    ValueHolder::Integer(v as i64)),
                None => Result::Err(String::from("Could not get u64 value."))
            };
        }
        Result::Err(String::from("Could not extract Integer from Number."))
    }

}

pub struct StringTransformer;