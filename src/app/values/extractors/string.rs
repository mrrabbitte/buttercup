use serde_json::Value;

use crate::app::values::extractors::{ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct StringExtractor;

impl ValueExtractor for StringExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match &input.value {
            Value::String(value) =>
                Result::Ok(
                    ValueHolder::String(
                        String::from(value))),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match serde_json::to_string(&input.value) {
            Ok(value) => Result::Ok(ValueHolder::String(value)),
            Err(_) => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax)),
        };
    }

}