use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError, Weekday};
use chrono_tz::Tz;
use num::FromPrimitive;
use serde_json::{Number, Value};

use crate::app::values::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;
use crate::app::values::wrappers::{WeekdayWrapper, Wrapper};

pub struct DayOfWeekExtractor;

impl DayOfWeekExtractor {

    fn ok(weekday: Weekday) -> Result<ValueHolder, ValueExtractionPolicy> {
        Result::Ok(
            ValueHolder::DayOfWeek(
                WeekdayWrapper::new(weekday)))
    }

}

impl ValueExtractor for DayOfWeekExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::Number(num_value) => {
                if num_value.is_u64() {
                    return match num_value.as_u64().and_then(Weekday::from_u64) {
                        Some(weekday) => DayOfWeekExtractor::ok(weekday),
                        None => Result::Err(ValueExtractionPolicy::Strict)
                    };
                }
                if num_value.is_i64() {
                    return match num_value.as_i64().and_then(Weekday::from_i64) {
                        Some(weekday) => DayOfWeekExtractor::ok(weekday),
                        None => Result::Err(ValueExtractionPolicy::Strict)
                    };
                }
                return Result::Err(ValueExtractionPolicy::Strict);
            },
            _ => Result::Err(ValueExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::String(str_val) => {
                return match str_val.to_lowercase().parse::<Weekday>() {
                    Ok(weekday) => DayOfWeekExtractor::ok(weekday),
                    Err(_) => Result::Err(ValueExtractionPolicy::Lax)
                };
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        };
    }

}