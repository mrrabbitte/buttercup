use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use serde_json::{Error, Value};

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::geolocation::{GeoCoordinates, GeoCoordinatesValueError};
use crate::app::values::ValueHolder;

pub struct GeoCoordinatesExtractor;

const GIVEN_LAT_LONG_NOT_VALID: &str =
    "Latitude should be in range [-90; 90], longitude should be in range [-180, 180].";

impl ValueExtractor for GeoCoordinatesExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::Object(map) => match GeoCoordinates::try_from(map) {
                Ok(coordinates) =>
                    Result::Ok(
                        ValueHolder::GeoCoordinates(coordinates)),
                Err(err) =>
                    Result::Err(
                        ValueExtractionError::GeoCoordinatesValueError(
                            ValueExtractionPolicy::Strict, err)),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(str_value) => {
                return match GeoCoordinates::from_str(str_value.as_str()) {
                    Ok(coordinates) =>
                        Result::Ok(ValueHolder::GeoCoordinates(coordinates)),
                    Err(err) => Result::Err(
                        ValueExtractionError::GeoCoordinatesValueError(
                            ValueExtractionPolicy::Lax, err)),
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
    fn test_serde() {
        let coordinates = GeoCoordinates::new(
            BigRational::from_f64(89.9999999).unwrap(),
            BigRational::from_f64(179.999999).unwrap())
            .unwrap();
        println!("{}", serde_json::to_string(&coordinates).unwrap());
    }

    #[test]
    fn test_strict() {
        let input_value = Value::from_str(r#"
                {
                "latitude" : "10.2",
                "longitude" : "23.3"
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
                BigRational::from_f64(10.2).unwrap(),
                BigRational::from_f64(23.3).unwrap())
                .unwrap();
        match value {
            ValueHolder::GeoCoordinates(coordinates) =>
                assert_eq!(expected, coordinates),
            _ => panic!("Invalid value type.")
        };
    }

    #[test]
    fn test_lax() {
        let input_value = Value::from_str(r#"
                "89.9999999,179.999999"
                "#)
            .unwrap();
        let input = ValueExtractorInput::new(
            &input_value,
            &ValueType::GeoCoordinates,
            &ValueExtractionPolicy::Lax);
        let result =
            GeoCoordinatesExtractor::lax_extract(&input);
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
    fn test_strict_failure_invalid() {

    }

    #[test]
    fn test_lax_failure_invalid() {

    }

}