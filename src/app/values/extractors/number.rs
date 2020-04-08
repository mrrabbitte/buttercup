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
                return match str_val.parse::<BigRational>() {
                    Ok(big_rational) =>
                        Result::Ok(
                            ValueHolder::Decimal(big_rational)),
                    Err(_) => Result::Err(
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
