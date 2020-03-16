use actix_web::http::header::IntoHeaderValue;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError, Weekday};
use num::FromPrimitive;
use serde_json::{Number, Value};

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::arguments::values::ValueHolder;

pub struct LocalDateTimeExtractor;

impl ValueExtractor for LocalDateTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
            Value::String(val) => {
                return match val.parse::<NaiveDateTime>() {
                    Ok(date_time) => Result::Ok(ValueHolder::LocalDateTime(date_time)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Strict),
                }
            },
            _ => Result::Err(ValueExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
            Value::Number(number) => LocalDateTimeExtractor::from_timestamp_millis(number),
            _ => Result::Err(ValueExtractionPolicy::Lax)
        };
    }

}

impl LocalDateTimeExtractor {

    fn from_timestamp_millis(value: &Number) -> Result<ValueHolder, ValueExtractionPolicy> {

    }

}

pub struct LocalDateExtractor;

impl ValueExtractor for LocalDateExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
            Value::String(val) => {
                return match val.parse::<NaiveDate>() {
                    Ok(date) => Result::Ok(ValueHolder::LocalDate(date)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Strict),
                }
            },
            _ => Result::Err(ValueExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        Result::Err(ValueExtractionPolicy::Lax)
    }

}

pub struct LocalTimeExtractor;

impl ValueExtractor for LocalTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
            Value::String(val) => {
                return match val.parse::<NaiveTime>() {
                    Ok(time) => Result::Ok(ValueHolder::LocalTime(time)),
                    Err(_) => Result::Err(ValueExtractionPolicy::Strict),
                }
            },
            _ => Result::Err(ValueExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        Result::Err(ValueExtractionPolicy::Lax)
    }

}

pub struct DayOfWeekExtractor;

impl ValueExtractor for DayOfWeekExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match &input.value {
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
        return match &input.value {
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