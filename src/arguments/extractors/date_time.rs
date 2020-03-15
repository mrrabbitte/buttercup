use chrono::{NaiveDateTime, ParseError, Weekday};
use serde_json::Value;

use crate::arguments::extractors::{ExtractionPolicy, ExtractorInput, ValueExtractor};
use crate::arguments::values::ValueHolder;
use actix_web::http::header::IntoHeaderValue;

pub struct LocalDateTimeExtractor;

impl ValueExtractor for LocalDateTimeExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.value {
            Value::String(val) => {
                return match val.parse::<NaiveDateTime>() {
                    Ok(date_time) => Result::Ok(ValueHolder::LocalDateTime(date_time)),
                    Err(_) => Result::Err(ExtractionPolicy::Strict),
                }
            },
            _ => Result::Err(ExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        unimplemented!()
    }

}

pub struct LocalDateExtractor;

impl ValueExtractor for LocalDateExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        unimplemented!()
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        unimplemented!()
    }

}

pub struct LocalTimeExtractor;

impl ValueExtractor for LocalTimeExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        unimplemented!()
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        unimplemented!()
    }

}

pub struct DayOfWeekExtractor;

impl ValueExtractor for DayOfWeekExtractor {

    fn strict_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.value {
            Value::Number(num_value) => {
                return match num_value.as_u64() {
                    Some(u64_val) => Result::Ok(ValueHolder::DayOfWeek(
                        u64_val.try_into())),
                    None => Result::Err(ExtractionPolicy::Strict)
                };
            },
            _ => Result::Err(ExtractionPolicy::Strict)
        };
    }

    fn lax_extract(input: &ExtractorInput) -> Result<ValueHolder, ExtractionPolicy> {
        return match &input.value {
            Value::String(str_val) => {
                let parse_result = val.to_lowercase().parse::<Weekday>();
                return match parse_result {
                    Ok(weekday) => Result::Ok(ValueHolder::DayOfWeek(weekday)),
                    Err(_) => Result::Err(ExtractionPolicy::Lax)
                };
            },
            _ => Result::Err(ExtractionPolicy::Lax)
        };
    }

}