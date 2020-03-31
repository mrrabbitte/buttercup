use std::str::FromStr;

use serde_json::{Error, Value};

use crate::app::values::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::geolocation::GeoCoordinates;
use crate::app::values::ValueHolder;

pub struct GeoCoordinatesExtractor;

impl ValueExtractor for GeoCoordinatesExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match serde_json::from_value::<GeoCoordinates>(input.value.clone()) {
            Ok(coordinates) => {
                if !coordinates.is_valid() {
                    return Result::Err(ValueExtractionPolicy::Strict);
                }
                Result::Ok(ValueHolder::GeoCoordinates(coordinates))
            },
            Err(_) => Result::Err(ValueExtractionPolicy::Strict),
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::String(str_value) => {
                return match GeoCoordinates::from_str(str_value.as_str()) {
                    Ok(coordinates) =>
                        Result::Ok(ValueHolder::GeoCoordinates(coordinates)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Lax),
                };
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        }
    }

}