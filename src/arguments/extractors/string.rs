use std::convert::TryInto;

use serde_json::Error;

use crate::arguments::extractors::{ExtractionPolicy, ExtractorInput, ValueExtractor};
use crate::arguments::values::ValueHolder;

pub struct StringExtractor;

impl ValueExtractor for StringExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match input.value.as_str() {
            Some(val) => Result::Ok(ValueHolder::String(val)),
            None => Result::Err(ExtractionPolicy::Strict),
        };
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        let value = &input.value;
        return match serde_json::to_string(value) {
            Ok(val) => Result::Ok(ValueHolder::String(val.as_str())),
            Err(_) => Result::Err(ExtractionPolicy::Lax),
        };
    }

}