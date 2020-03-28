use std::convert::TryInto;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError, Weekday};
use chrono_tz::Tz;
use num::FromPrimitive;
use serde_json::{Number, Value};

use crate::app::arguments::extractors::{ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;
use crate::app::values::zoned_date_time::{ZonedDateTime, ZonedDateTimeParsingError};

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

impl ValueExtractor for ZonedDateTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        let from_value_result: Result<ZonedDateTime, ZonedDateTimeParsingError> =
            input.value.try_into();
        return match from_value_result {
            Ok(zdt) => Result::Ok(ValueHolder::ZonedDateTime(zdt)),
            Err(_) => Result::Err(ValueExtractionPolicy::Strict),
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionPolicy> {
        return match input.value {
            Value::String(str_val) => match str_val.parse::<ZonedDateTime>() {
                Ok(zdt) => Result::Ok(ValueHolder::ZonedDateTime(zdt)),
                Err(_) => Result::Err(ValueExtractionPolicy::Lax),
            },
            _ => Result::Err(ValueExtractionPolicy::Lax)
        };
    }

}