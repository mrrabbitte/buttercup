use serde_json::Value;

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct BooleanExtractor;

impl ValueExtractor for BooleanExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(*bool_val)),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::Bool(bool_val) => Result::Ok(ValueHolder::Boolean(*bool_val)),
            Value::String(str_val) => {
              match str_val.parse::<bool>() {
                  Ok(bool_val) => Result::Ok(ValueHolder::Boolean(bool_val)),
                  Err(_) => Result::Err(
                      ValueExtractionError::ParsingError(
                          ValueExtractionPolicy::Lax, ParsingValueSource::String))
              }
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        }
    }
}