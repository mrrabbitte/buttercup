use serde_json::Value;

use crate::email::Email;
use crate::extractors::{ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::ValueHolder;

pub struct EmailValueExtractor;

impl ValueExtractor for EmailValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match Email::new(str_val) {
                Ok(email) => Result::Ok(ValueHolder::Email(email)),
                Err(err) => Result::Err(
                    ValueExtractionError::EmailParsingError(
                        ValueExtractionPolicy::Strict, err.to_string())),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(_input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(ValueExtractionError::PolicyNotSupported(ValueExtractionPolicy::Lax))
    }

}
