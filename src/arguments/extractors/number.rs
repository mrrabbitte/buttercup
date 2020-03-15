use num::{BigInt, BigRational};
use serde_json::Value;

use crate::arguments::extractors::{ExtractionPolicy, ExtractorInput, ValueExtractor};
use crate::arguments::values::ValueHolder;

pub struct DecimalExtractor;

impl ValueExtractor for DecimalExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        let val = &input.value;
        if val.is_f64() {
            let opt_f64 = val.as_f64();
            return match opt_f64 {
                Some(v) => Result::Ok(
                    ValueHolder::Decimal(BigRational::from(v))),
                None => Result::Err(ExtractionPolicy::Strict)
            };
        }
        Result::Err(ExtractionPolicy::Strict)
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.value {
            Value::Number(num_val) => {
                if num_val.is_u64() {
                    return Result::Ok(
                        ValueHolder::Decimal(
                            BigRational::from(num_val.as_u64().unwrap())));
                }
                if num_val.is_i64() {
                    return Result::Ok(
                        ValueHolder::Decimal(
                            BigRational::from(num_val.as_i64().unwrap())));
                }
                Result::Err(ExtractionPolicy::Lax)
            },
            Value::String(str_val) =>
                Result::Ok(
                    ValueHolder::Decimal(
                        BigRational::from(str_val))),
            _ => Result::Err(ExtractionPolicy::Lax)
        };
    }

}

pub struct IntegerExtractor;

impl ValueExtractor for IntegerExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, String> {
        let val = &input.value;
        if val.is_i64() {
            let opt_i64 = val.as_i64();
            return match opt_i64 {
                Some(v) => Result::Ok(
                    ValueHolder::Integer(BigInt::from(v))),
                None => Result::Err(String::from("Could not get i64 value."))
            };
        }
        if val.is_u64() {
            let opt_u64 = val.as_u64();
            return match opt_u64 {
                Some(v) => Result::Ok(
                    ValueHolder::Integer(BigInt::from(v))),
                None => Result::Err(String::from("Could not get u64 value."))
            };
        }
        Result::Err(format!("Could not transform value: {:?} to Integer.", val))
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, String> {
        let val = &input.value;
        return match val {
            Value::Number(num_val) => {
                if val.is_f64() {
                    return Result::Ok(
                        ValueHolder::Integer(BigInt::from(num_val.as_f64().unwrap())));
                }
                Result::Err(String::from("Could not extract Integer from value."))},
            Value::String(_) => Result::Ok(
                ValueHolder::Integer(BigInt::from(v))),
            _ => Result::Err(String::from("Could not extract Integer from value."))
        };
    }

}
