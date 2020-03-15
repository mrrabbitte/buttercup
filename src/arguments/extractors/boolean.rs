use serde_json::Value;

use crate::arguments::extractors::{ExtractionPolicy, ExtractorInput, ValueExtractor};
use crate::arguments::values::ValueHolder;

pub struct BooleanExtractor;

impl ValueExtractor for BooleanExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, String> {
        match input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(bool_val)),
            _ => Result::Err(String::from("Could not parse value with strict policy."))
        }
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, String> {
        match &input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(*bool_val)),
            Value::String(str_val) => {
              match str_val.parse::<bool>() {
                  Ok(bool_val) => Result::Ok(ValueHolder::Boolean(bool_val)),
                  Err(_) => Result::Err(String::from("Could not parse value with lax policy."))
              }
            },
            _ => Result::Err(String::from("Could not parse value with lax policy."))
        }
    }
}