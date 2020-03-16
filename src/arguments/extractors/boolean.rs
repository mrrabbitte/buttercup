use serde_json::Value;

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::values::ValueHolder;

pub struct BooleanExtractor;

impl ValueExtractor for BooleanExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        match input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(*bool_val)),
            _ => Result::Err(ValueExtractionPolicy::Strict)
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        match input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(*bool_val)),
            Value::String(str_val) => {
              match str_val.parse::<bool>() {
                  Ok(bool_val) => Result::Ok(ValueHolder::Boolean(bool_val)),
                  Err(_) => Result::Err(ValueExtractionPolicy::Lax)
              }
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        }
    }
}