use std::convert::TryFrom;
use std::str::FromStr;

use serde_json::Value;

use crate::app::values::extractors::{ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::geolocation::{GeoCoordinates, GeoCoordinatesValueError};
use crate::app::values::ValueHolder;

pub struct GeoCoordinatesExtractor;

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
            BigRational::from_f64(89.99999991).unwrap(),
            BigRational::from_f64(-179.9999991).unwrap())
            .unwrap();
        assert_eq!(coordinates,
                   serde_json::from_str(
                       serde_json::to_string(&coordinates)
                           .unwrap().as_str())
                       .unwrap());
    }

    #[test]
    fn test_strict() {
        extract_and_check_ok(r#"
                {
                "latitude" : "10.2",
                "longitude" : "89.999999"
                }
                "#,
                             GeoCoordinates::new(
                              BigRational::from_f64(10.2).unwrap(),
                              BigRational::from_f64(89.999999).unwrap())
                              .unwrap(),
                             GeoCoordinatesExtractor::strict_extract,
                             ValueExtractionPolicy::Strict);
        extract_and_check_ok(r#"
                {
                "lat" : -10.2,
                "long" : 23.3
                }
                "#,
                             GeoCoordinates::new(
                              BigRational::from_f64(-10.2).unwrap(),
                              BigRational::from_f64(23.3).unwrap())
                              .unwrap(),
                             GeoCoordinatesExtractor::strict_extract,
                             ValueExtractionPolicy::Strict);
        extract_and_check_ok(r#"
                {
                "lat" : -10.9999999999999,
                "lon" : -16
                }
                "#,
                             GeoCoordinates::new(
                              BigRational::from_f64(-10.9999999999999).unwrap(),
                              BigRational::from_f64(-16.0).unwrap())
                              .unwrap(),
                             GeoCoordinatesExtractor::strict_extract,
                             ValueExtractionPolicy::Strict);
        extract_and_check_ok(r#"
                {
                "lat" : -10.124436341231,
                "lon" : 16
                }
                "#,
                             GeoCoordinates::new(
                              BigRational::from_f64(-10.124436341231).unwrap(),
                              BigRational::from_f64(16.0).unwrap())
                              .unwrap(),
                             GeoCoordinatesExtractor::strict_extract,
                             ValueExtractionPolicy::Strict);
    }

    #[test]
    fn test_lax() {
        extract_and_check_ok(r#"
                "89.9999999,179.999999"
                "#,
                             GeoCoordinates::new(
                              BigRational::from_f64(89.9999999).unwrap(),
                              BigRational::from_f64(179.999999).unwrap())
                              .unwrap(),
                             GeoCoordinatesExtractor::lax_extract,
                             ValueExtractionPolicy::Lax);
        extract_and_check_ok(r#"
                "-89.9999999999,-179.9999999999"
                "#,
                             GeoCoordinates::new(
                                 BigRational::from_f64(-89.9999999999).unwrap(),
                                 BigRational::from_f64(-179.9999999999).unwrap())
                                 .unwrap(),
                             GeoCoordinatesExtractor::lax_extract,
                             ValueExtractionPolicy::Lax);
        extract_and_check_ok(r#"
                "-90.0000,-180.00000"
                "#,
                             GeoCoordinates::new(
                                 BigRational::from_f64(-90.0).unwrap(),
                                 BigRational::from_f64(-180.0).unwrap())
                                 .unwrap(),
                             GeoCoordinatesExtractor::lax_extract,
                             ValueExtractionPolicy::Lax);
    }

    #[test]
    fn test_strict_failure_invalid() {
        extract_and_check_err(r#"
                {
                "lat" : -90.001,
                "lon" : 16
                }
                "#,
                             ValueExtractionError::GeoCoordinatesValueError(
                                 ValueExtractionPolicy::Strict,
                                 GeoCoordinatesValueError::InvalidLatitude),
                             GeoCoordinatesExtractor::strict_extract,
                             ValueExtractionPolicy::Strict);
        extract_and_check_err(r#"
                {
                "lat" : -90.00,
                "lon" : 181.00
                }
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Strict,
                                  GeoCoordinatesValueError::InvalidLongitude),
                              GeoCoordinatesExtractor::strict_extract,
                              ValueExtractionPolicy::Strict);
        extract_and_check_err(r#"
                {
                "lon" : 16
                }
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Strict,
                                  GeoCoordinatesValueError::MissingLatitude),
                              GeoCoordinatesExtractor::strict_extract,
                              ValueExtractionPolicy::Strict);
        extract_and_check_err(r#"
                {
                "lat" : -90.001
                }
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Strict,
                                  GeoCoordinatesValueError::MissingLongitude),
                              GeoCoordinatesExtractor::strict_extract,
                              ValueExtractionPolicy::Strict);
        extract_and_check_err(r#"
                "90,180"
                "#,
                              ValueExtractionError::InvalidValueTypeError(
                                  ValueExtractionPolicy::Strict),
                              GeoCoordinatesExtractor::strict_extract,
                              ValueExtractionPolicy::Strict);
    }

    #[test]
    fn test_lax_failure_invalid() {
        extract_and_check_err(r#"
                "90;180"
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Lax,
                                  GeoCoordinatesValueError::InvalidCommaSeparatedStructure),
                              GeoCoordinatesExtractor::lax_extract,
                              ValueExtractionPolicy::Lax);
        extract_and_check_err(r#"
                "91.0,180"
                "#,
                             ValueExtractionError::GeoCoordinatesValueError(
                                 ValueExtractionPolicy::Lax,
                                 GeoCoordinatesValueError::InvalidLatitude),
                             GeoCoordinatesExtractor::lax_extract,
                             ValueExtractionPolicy::Lax);
        extract_and_check_err(r#"
                "90,-180.01"
                "#,
                             ValueExtractionError::GeoCoordinatesValueError(
                                 ValueExtractionPolicy::Lax,
                                 GeoCoordinatesValueError::InvalidLongitude),
                             GeoCoordinatesExtractor::lax_extract,
                             ValueExtractionPolicy::Lax);
        extract_and_check_err(r#"
                ",180"
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Lax,
                                  GeoCoordinatesValueError::InvalidLatitude),
                              GeoCoordinatesExtractor::lax_extract,
                              ValueExtractionPolicy::Lax);
        extract_and_check_err(r#"
                "0,"
                "#,
                              ValueExtractionError::GeoCoordinatesValueError(
                                  ValueExtractionPolicy::Lax,
                                  GeoCoordinatesValueError::InvalidLongitude),
                              GeoCoordinatesExtractor::lax_extract,
                              ValueExtractionPolicy::Lax);
    }

    fn extract<F>(value: &str,
                  extraction: F,
                  policy: ValueExtractionPolicy) -> Result<ValueHolder, ValueExtractionError>
        where F: Fn(&ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let input_value = Value::from_str(value)
            .unwrap();
        let input = ValueExtractorInput::new(
            &input_value,
            &ValueType::GeoCoordinates,
            &policy);
        extraction(&input)
    }

    fn extract_and_check_ok<F>(value: &str,
                               expected: GeoCoordinates,
                               extraction: F,
                               policy: ValueExtractionPolicy)
        where F: Fn(&ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match extract(value, extraction, policy).unwrap() {
            ValueHolder::GeoCoordinates(coordinates) =>
                assert_eq!(expected, coordinates),
            _ => panic!("Invalid value type.")
        };
    }

    fn extract_and_check_err<F>(value: &str,
                                expected_error: ValueExtractionError,
                                extraction: F,
                                policy: ValueExtractionPolicy)
        where F: Fn(&ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let result = extract(value, extraction, policy);
        assert_eq!(true, result.is_err(), "{}", format!("{:?}", result));
        match result {
            Ok(_) => panic!("Should be an error."),
            Err(err) => assert_eq!(expected_error, err),
        }
    }

}