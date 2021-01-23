
use num::{BigInt, BigRational, FromPrimitive};

use serde_json::Value;

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct DecimalExtractor;

impl ValueExtractor for DecimalExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let val = input.value;
        if val.is_f64() {
            return match val.as_f64().and_then(BigRational::from_f64) {
                Some(v) => Result::Ok(ValueHolder::Decimal(v)),
                None => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Strict, ParsingValueSource::F64))
            };
        }
        Result::Err(ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Strict))
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::Number(num_val) => {
                if num_val.is_u64() {
                    return match num_val.as_u64().and_then(BigRational::from_u64) {
                        Some(v) => Result::Ok(ValueHolder::Decimal(v)),
                        None => Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Lax, ParsingValueSource::U64))
                    };
                }
                if num_val.is_i64() {
                    return match num_val.as_i64().and_then(BigRational::from_i64) {
                        Some(v) => Result::Ok(ValueHolder::Decimal(v)),
                        None => Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Lax, ParsingValueSource::I64))
                    };
                }
                Result::Err(ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Lax))
            },
            Value::String(str_val) => {
                return match str_val
                    .parse::<f64>() {
                    Ok(f64_val) => match BigRational::from_f64(f64_val) {
                        None =>  Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Lax, ParsingValueSource::String)),
                        Some(big_rational) => Result::Ok(
                            ValueHolder::Decimal(big_rational)),
                    },
                    Err(_) =>  Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Lax, ParsingValueSource::String)),
                }
            },
            _ =>
                Result::Err(
                    ValueExtractionError::InvalidValueTypeError(
                        ValueExtractionPolicy::Lax))
        };
    }

}

pub struct IntegerExtractor;

impl ValueExtractor for IntegerExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let val = input.value;
        if val.is_i64() {
            return match val.as_i64().and_then(BigInt::from_i64) {
                Some(v) => Result::Ok(ValueHolder::Integer(v)),
                None => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Strict, ParsingValueSource::I64))
            };
        }
        if val.is_u64() {
            return match val.as_u64().and_then(BigInt::from_u64) {
                Some(v) => Result::Ok(ValueHolder::Integer(v)),
                None => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Strict, ParsingValueSource::U64))
            };
        }
        Result::Err(
            ValueExtractionError::InvalidValueTypeError(
                ValueExtractionPolicy::Strict))
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let val = input.value;
        return match val {
            Value::Number(num_val) => {
                if val.is_f64() {
                    return match num_val.as_f64().and_then(BigInt::from_f64) {
                        Some(v) => Result::Ok(ValueHolder::Integer(v)),
                        None => Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Lax, ParsingValueSource::F64))
                    };
                }
                Result::Err(
                    ValueExtractionError::InvalidValueTypeError(
                        ValueExtractionPolicy::Strict))},
            Value::String(str_val) => {
                return match str_val.parse::<BigInt>() {
                    Ok(v) => Result::Ok(ValueHolder::Integer(v)),
                    Err(_) => Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Lax, ParsingValueSource::String)),
                };
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        };
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::app::values::ValueType;

    use super::*;

    #[test]
    fn test_decimal_strict() {
        extract_and_check_ok(r#"
                0.0
                "#, DecimalExtractor::strict_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Strict,
                             ValueHolder::Decimal(BigRational::from_f64(0.0)
                                 .unwrap()));
        extract_and_check_ok(r#"
                -0.0
                "#, DecimalExtractor::strict_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Strict,
                             ValueHolder::Decimal(BigRational::from_f64(0.0)
                                 .unwrap()));
        extract_and_check_ok(r#"
                -0.001
                "#, DecimalExtractor::strict_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Strict,
                             ValueHolder::Decimal(BigRational::from_f64(-0.001)
                                 .unwrap()));
    }

    #[test]
    fn test_decimal_lax() {
        extract_and_check_ok(r#"
                0
                "#, DecimalExtractor::lax_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Lax,
                             ValueHolder::Decimal(BigRational::from_f64(0.0)
                                 .unwrap()));
        extract_and_check_ok(r#"
                -0
                "#, DecimalExtractor::lax_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Lax,
                             ValueHolder::Decimal(BigRational::from_f64(0.0)
                                 .unwrap()));
        extract_and_check_ok(r#"
                "-0.00000000000001"
                "#, DecimalExtractor::lax_extract,
                             &ValueType::Decimal,
                             ValueExtractionPolicy::Lax,
                             ValueHolder::Decimal(
                                 BigRational::from_f64(-0.00000000000001).unwrap()));
    }

    #[test]
    fn test_integer_strict() {
        extract_and_check_ok(r#"
                1012321311231231012
                "#, IntegerExtractor::strict_extract,
                             &ValueType::Integer,
                             ValueExtractionPolicy::Strict,
                             ValueHolder::Integer(
                                 BigInt::from(1012321311231231012 as u64)));
        extract_and_check_ok(r#"
                -1012321311231231012
                "#, IntegerExtractor::strict_extract,
                             &ValueType::Integer,
                             ValueExtractionPolicy::Strict,
                             ValueHolder::Integer(
                                 BigInt::from(-1012321311231231012 as i64)));
    }

    #[test]
    fn test_integer_lax() {
        extract_and_check_ok(r#"
                -10123213112312310.0
                "#, IntegerExtractor::lax_extract,
                             &ValueType::Integer,
                             ValueExtractionPolicy::Lax,
                             ValueHolder::Integer(
                                 BigInt::from(-10123213112312310 as i64)));
        extract_and_check_ok(r#"
                "-101232131123123100000000"
                "#, IntegerExtractor::lax_extract,
                             &ValueType::Integer,
                             ValueExtractionPolicy::Lax,
                             ValueHolder::Integer(
                                 BigInt::from(-101232131123123100000000 as i128)));
    }

    fn extract<F>(value: &str,
                  extraction: F,
                  value_type: &ValueType,
                  policy: ValueExtractionPolicy) -> Result<ValueHolder, ValueExtractionError>
        where F: Fn(&ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let input_value = Value::from_str(value)
            .unwrap();
        let input = ValueExtractorInput::new(
            &input_value,
            value_type,
            &policy);
        extraction(&input)
    }

    fn extract_and_check_ok<F>(value: &str,
                               extraction: F,
                               value_type: &ValueType,
                               policy: ValueExtractionPolicy,
                               expected: ValueHolder)
        where F: Fn(&ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let result = extract(value, extraction, value_type, policy).unwrap();
        assert_eq!(expected, result);
    }

}
