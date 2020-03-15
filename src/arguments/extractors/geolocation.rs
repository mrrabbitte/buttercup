use std::str::FromStr;

use serde_json::{Error, Value};

use crate::arguments::extractors::{ExtractionPolicy, ExtractorInput, ValueExtractor};
use crate::arguments::values::{GeoCoordinates, ValueHolder};

pub struct LatLongExtractor;

impl ValueExtractor for LatLongExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match serde_json::from_value(*input.value) {
            Ok(coordinates) => Result::Ok(ValueHolder::LatLong(coordinates)),
            Err(_) => Result::Err(ExtractionPolicy::Strict),
        }
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.value {
            Value::String(str_value) => {
                return match GeoCoordinates::from_str(str_value.as_str()) {
                    Ok(coordinates) => Result::Ok(ValueHolder::LatLong(coordinates)),
                    Err(_) => Result::Err(ExtractionPolicy::Lax),
                };
            },
            _ => Result::Err(ExtractionPolicy::Lax)
        }
    }

}