use std::str::FromStr;

use serde_json::{Error, Value};

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::arguments::values::{GeoCoordinates, ValueHolder};

pub struct LatLongExtractor;

impl ValueExtractor for LatLongExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match serde_json::from_value(input.value.clone()) {
            Ok(coordinates) => Result::Ok(ValueHolder::LatLong(coordinates)),
            Err(_) => Result::Err(ValueExtractionPolicy::Strict),
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::String(str_value) => {
                return match GeoCoordinates::from_str(str_value.as_str()) {
                    Ok(coordinates) => Result::Ok(ValueHolder::LatLong(coordinates)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Lax),
                };
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        }
    }

}