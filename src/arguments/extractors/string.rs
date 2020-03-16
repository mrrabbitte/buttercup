use std::borrow::Borrow;
use std::convert::TryInto;

use serde_json::{Error, Value};

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::arguments::values::ValueHolder;

pub struct StringExtractor;

impl ValueExtractor for StringExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
            Value::String(value) =>
                Result::Ok(
                    ValueHolder::String(
                        String::from(value))),
            _ => Result::Err(ValueExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match serde_json::to_string(&input.value) {
            Ok(value) => Result::Ok(ValueHolder::String(value)),
            Err(_) => Result::Err(ValueExtractionPolicy::Lax),
        };
    }

}