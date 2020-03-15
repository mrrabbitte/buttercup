use serde_json::{Number, Value};

use crate::arguments::extractors::boolean::BooleanExtractor;
use crate::arguments::extractors::date_time::{DayOfWeekExtractor, LocalDateExtractor, LocalDateTimeExtractor, LocalTimeExtractor};
use crate::arguments::extractors::geolocation::LatLongExtractor;
use crate::arguments::extractors::number::{DecimalExtractor, IntegerExtractor};
use crate::arguments::extractors::string::StringExtractor;
use crate::arguments::values::ValueHolder;
use crate::arguments::ValueType;

pub mod boolean;
pub mod date_time;
pub mod geolocation;
pub mod number;
pub mod string;

#[derive(Display)]
pub enum ExtractionPolicy {

    Strict,
    Lax

}

#[derive(Display)]
pub struct ExtractorInput {

    value: Value,
    argument_type: ValueType,
    policy: ExtractionPolicy

}

pub struct Extractor;

impl Extractor {

    pub fn extract(input: &ExtractorInput) -> Result<ValueHolder, String> {
        if input.value.is_null() {
            return Result::Err(String::from("Got null value."));
        }
        let extraction_result = match &input.argument_type {
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
        return match extraction_result {
            Ok(value) => Result::Ok(value),
            Err(policy) => Result::Err(
                format!("Could not extract value from: {:?} with: {:?} policy",
                        input, policy)),
        };
    }

}

pub trait ValueExtractor {

    fn extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.policy {
            ExtractionPolicy::Strict => Self::strict_extract(input),
            ExtractionPolicy::Lax => {
                let strict_result = Self::strict_extract(input);
                if strict_result.is_ok() {
                    return strict_result;
                }
                return Self::lax_extract(input);
            },
        }
    }

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy>;
    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy>;

}

