use chrono::Weekday;

use num::FromPrimitive;
use serde_json::Value;

use crate::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::ValueHolder;
use crate::wrappers::{WeekdayWrapper, Wrapper};

pub struct DayOfWeekExtractor;

impl DayOfWeekExtractor {

    fn ok(weekday: Weekday) -> Result<ValueHolder, ValueExtractionError> {
        Result::Ok(
            ValueHolder::DayOfWeek(
                WeekdayWrapper::new(weekday)))
    }

}

impl ValueExtractor for DayOfWeekExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::Number(num_value) => {
                if num_value.is_u64() {
                    return match num_value.as_u64().and_then(Weekday::from_u64) {
                        Some(weekday) => DayOfWeekExtractor::ok(weekday),
                        None => Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Strict,
                                ParsingValueSource::U64))
                    };
                }
                if num_value.is_i64() {
                    return match num_value.as_i64().and_then(Weekday::from_i64) {
                        Some(weekday) => DayOfWeekExtractor::ok(weekday),
                        None => Result::Err(
                            ValueExtractionError::ParsingError(
                                ValueExtractionPolicy::Strict,
                                ParsingValueSource::I64))
                    };
                }
                Result::Err(
                    ValueExtractionError::InvalidValueTypeError(
                        ValueExtractionPolicy::Strict))
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(str_val) => {
                return match str_val.to_lowercase().parse::<Weekday>() {
                    Ok(weekday) => DayOfWeekExtractor::ok(weekday),
                    Err(_) => Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Strict,
                            ParsingValueSource::String))
                };
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        };
    }

}