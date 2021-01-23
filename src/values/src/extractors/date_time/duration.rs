use std::time::Duration;

use serde_json::{Number, Value};

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct DurationExtractor;

impl DurationExtractor {

    fn from_milli(value: &Number) -> Result<ValueHolder, ValueExtractionError> {
        if value.is_u64() {
            return match value
                .as_u64()
                .and_then(DurationExtractor::from_u64) {
                Some(value_holder) => Result::Ok(value_holder),
                None =>  Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Lax, ParsingValueSource::U64))
            };
        }
        Result::Err(
            ValueExtractionError::InvalidValueTypeError(
                ValueExtractionPolicy::Lax
            )
        )
    }

    fn from_u64(millis: u64) -> Option<ValueHolder> {
        Option::Some(
            ValueHolder::Duration(
                Duration::from_millis(millis)
            )
        )
    }

}

impl ValueExtractor for DurationExtractor {
    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::Number(number) =>
                DurationExtractor::from_milli(number),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(_: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(ValueExtractionError::PolicyNotSupported(ValueExtractionPolicy::Lax))
    }
}