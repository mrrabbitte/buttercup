use serde_json::{Number, Value};

use crate::arguments::extractors::boolean::BooleanExtractor;
use crate::arguments::extractors::date_time::{DayOfWeekExtractor, LocalDateExtractor, LocalDateTimeExtractor, LocalTimeExtractor};
use crate::arguments::extractors::geolocation::LatLongExtractor;
use crate::arguments::extractors::number::{DecimalExtractor, IntegerExtractor};
use crate::arguments::extractors::string::StringExtractor;
use crate::values::{ValueHolder, ValueType};

pub mod boolean;
pub mod date_time;
pub mod geolocation;
pub mod number;
pub mod string;

#[derive(Debug)]
pub enum ValueExtractionPolicy {

    Strict,
    Lax

}

#[derive(Debug)]
pub struct ValueExtractorInput<'a> {

    value: &'a Value,
    argument_type: &'a ValueType,
    policy: &'a ValueExtractionPolicy

}

impl<'a> ValueExtractorInput<'a> {

    pub fn new(value: &'a Value,
               argument_type: &'a ValueType,
               policy: &'a ValueExtractionPolicy) -> ValueExtractorInput<'a> {
        ValueExtractorInput {
            value,
            argument_type,
            policy
        }
    }

}

pub struct ArgumentValueExtractor;

impl ArgumentValueExtractor {

    pub fn extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        if input.value.is_null() {
            return Result::Err(ValueExtractionPolicy::Lax);
        }
        return match &input.argument_type {
            ValueType::Boolean => BooleanExtractor::extract(input),
            ValueType::String => StringExtractor::extract(input),
            ValueType::Decimal => DecimalExtractor::extract(input),
            ValueType::Integer => IntegerExtractor::extract(input),
            ValueType::LocalDateTime => LocalDateTimeExtractor::extract(input),
            ValueType::LocalDate => LocalDateExtractor::extract(input),
            ValueType::LocalTime => LocalTimeExtractor::extract(input),
            ValueType::DayOfWeek => DayOfWeekExtractor::extract(input),
            ValueType::LatLong => LatLongExtractor::extract(input)
        };
    }

}

pub trait ValueExtractor {

    fn extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.policy {
            ValueExtractionPolicy::Strict => Self::strict_extract(input),
            ValueExtractionPolicy::Lax => {
                let strict_result = Self::strict_extract(input);
                if strict_result.is_ok() {
                    return strict_result;
                }
                return Self::lax_extract(input);
            },
        }
    }

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy>;
    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy>;

}

