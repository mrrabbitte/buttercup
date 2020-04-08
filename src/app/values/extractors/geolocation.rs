use std::str::FromStr;

use serde_json::{Error, Value};

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::geolocation::GeoCoordinates;
use crate::app::values::ValueHolder;

pub struct GeoCoordinatesExtractor;

const GIVEN_LAT_LONG_NOT_VALID: &str =
    "Latitude should be in range [-90; 90], longitude should be in range [-180, 180].";

impl ValueExtractor for GeoCoordinatesExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match serde_json::from_value::<GeoCoordinates>(input.value.clone()) {
            Ok(coordinates) => {
                if !coordinates.is_valid() {
                    return Result::Err(
                        ValueExtractionError::InvalidValueError(
                            ValueExtractionPolicy::Strict,
                            GIVEN_LAT_LONG_NOT_VALID.to_string()));
                }
                Result::Ok(ValueHolder::GeoCoordinates(coordinates))
            },
            Err(_) => Result::Err(
                ValueExtractionError::ParsingError(
                    ValueExtractionPolicy::Strict, ParsingValueSource::Json)),
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(str_value) => {
                return match GeoCoordinates::from_str(str_value.as_str()) {
                    Ok(coordinates) =>
                        Result::Ok(ValueHolder::GeoCoordinates(coordinates)),
                    Err(_) => Result::Err(ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Lax, ParsingValueSource::String)),
                };
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        }
    }

}

#[cfg(test)]
mod tests {
    use num::FromPrimitive;
    use num_rational::BigRational;

    use crate::app::values::ValueType;

    use super::*;

    #[test]
    fn test_strict() {
        let input_value = Value::from_str(r#"
                {
                "latitude" : 89.9999999,
                "longitude" : 179.999999
                }
                "#)
            .unwrap();
        let input = ValueExtractorInput::new(
            &input_value,
            &ValueType::GeoCoordinates,
            &ValueExtractionPolicy::Strict);
        let result =
            GeoCoordinatesExtractor::strict_extract(&input);
        let value = result.unwrap();
        let expected =
            GeoCoordinates::new(
                BigRational::from_f64(89.9999999).unwrap(),
                BigRational::from_f64(179.999999).unwrap())
                .unwrap();
        match value {
            ValueHolder::GeoCoordinates(coordinates) =>
                assert_eq!(expected, coordinates),
            _ => panic!("Invalid value type.")
        };
    }

    #[test]
    fn test_lax() {

    }

    #[test]
    fn test_strict_failure_invalid() {

    }

    #[test]
    fn test_lax_failure_invalid() {

    }

}