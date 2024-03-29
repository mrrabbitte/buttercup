use std::convert::TryInto;

use chrono_tz::Tz;
use serde_json::Value;

use crate::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::ValueHolder;
use crate::wrappers::{TzWrapper, Wrapper};
use crate::zoned_date_time::{ZonedDateTime, ZonedDateTimeParsingError};

pub struct TimezoneExtractor;

impl ValueExtractor for TimezoneExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(str_val) => return match str_val.parse::<Tz>() {
                Ok(tz) => Result::Ok(ValueHolder::TimeZone(TzWrapper::new(tz))),
                Err(_) => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Strict,
                        ParsingValueSource::String)),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(_input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(ValueExtractionError::PolicyNotSupported(ValueExtractionPolicy::Lax))
    }

}

pub struct ZonedDateTimeExtractor;

impl ValueExtractor for ZonedDateTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        let from_value_result: Result<ZonedDateTime, ZonedDateTimeParsingError> =
            input.value.try_into();
        return match from_value_result {
            Ok(zdt) => Result::Ok(ValueHolder::ZonedDateTime(zdt)),
            Err(err) =>
                Result::Err(
                    ValueExtractionError::ZonedDateTimeParsingError(
                        ValueExtractionPolicy::Strict, err)),
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(str_val) => match str_val.parse::<ZonedDateTime>() {
                Ok(zdt) => Result::Ok(ValueHolder::ZonedDateTime(zdt)),
                Err(err) => Result::Err(
                    ValueExtractionError::ZonedDateTimeParsingError(
                        ValueExtractionPolicy::Lax, err)),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        };
    }

}