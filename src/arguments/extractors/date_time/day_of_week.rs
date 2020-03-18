use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError, Weekday};
use chrono_tz::Tz;
use num::FromPrimitive;
use serde_json::{Number, Value};

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::values::ValueHolder;

pub struct DayOfWeekExtractor;

impl ValueExtractor for DayOfWeekExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::Number(num_value) => {
                if num_value.is_u64() {
                    return match num_value.as_u64().and_then(Weekday::from_u64) {
                        Some(weekday) => Result::Ok(ValueHolder::DayOfWeek(weekday)),
                        None => Result::Err(ValueExtractionPolicy::Strict)
                    };
                }
                if num_value.is_i64() {
                    return match num_value.as_i64().and_then(Weekday::from_i64) {
                        Some(weekday) => Result::Ok(ValueHolder::DayOfWeek(weekday)),
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
                    Ok(weekday) => Result::Ok(ValueHolder::DayOfWeek(weekday)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Lax)
                };
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        };
    }

}