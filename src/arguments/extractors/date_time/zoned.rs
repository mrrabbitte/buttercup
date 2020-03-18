use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError, Weekday};
use chrono_tz::Tz;
use num::FromPrimitive;
use serde_json::{Number, Value};

use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::values::ValueHolder;

pub struct TimezoneExtractor;

impl ValueExtractor for TimezoneExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::String(str_val) => return match str_val.parse::<Tz>() {
                Ok(tz) => Result::Ok(ValueHolder::TimeZone(tz)),
                Err(_) => Result::Err(ValueExtractionPolicy::Strict),
            },
            _ => Result::Err(ValueExtractionPolicy::Strict)
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        Result::Err(ValueExtractionPolicy::Lax)
    }

}

pub struct ZonedDateTimeExtractor;

