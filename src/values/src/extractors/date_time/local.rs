use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use serde_json::{Number, Value};

use crate::app::values::extractors::{ParsingValueSource, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct LocalDateTimeExtractor;

impl ValueExtractor for LocalDateTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(val) => {
                return match val.parse::<NaiveDateTime>() {
                    Ok(date_time) => Result::Ok(ValueHolder::LocalDateTime(date_time)),
                    Err(_) => Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Strict,
                            ParsingValueSource::String)),
                }
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::Number(number) =>
                LocalDateTimeExtractor::from_timestamp_milli(number),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        };
    }

}

impl LocalDateTimeExtractor {

    fn from_timestamp_milli(value: &Number) -> Result<ValueHolder, ValueExtractionError> {
        if value.is_i64() {
            return match value
                .as_i64()
                .and_then(LocalDateTimeExtractor::from_timestamp_ms_i64) {
                Some(value_holder) => Result::Ok(value_holder),
                None => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Lax, ParsingValueSource::I64))
            };
        }
        if value.is_u64() {
            return match value
                .as_u64()
                .and_then(LocalDateTimeExtractor::from_timestamp_ms_u64) {
                Some(value_holder) => Result::Ok(value_holder),
                None =>  Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Lax, ParsingValueSource::U64))
            };
        }
        if value.is_f64() {
            return match value
                .as_f64()
                .and_then(LocalDateTimeExtractor::from_timestamp_ms_f64) {
                Some(value_holder) => Result::Ok(value_holder),
                None => Result::Err(
                    ValueExtractionError::ParsingError(
                        ValueExtractionPolicy::Lax, ParsingValueSource::F64))
            };
        }
        Result::Err(
            ValueExtractionError::InvalidValueTypeError(
                ValueExtractionPolicy::Lax))
    }

    fn from_timestamp_ms_i64(ts: i64) -> Option<ValueHolder> {
        let nanos = ((ts % 1000) * 1_000_000) as u32;
        let seconds = ts / 1000;
        Option::Some(
            ValueHolder::LocalDateTime(
                NaiveDateTime::from_timestamp(seconds, nanos )))
    }

    fn from_timestamp_ms_u64(ts: u64) -> Option<ValueHolder> {
        let nanos = ((ts % 1000) * 1_000_000) as u32;
        let seconds = (ts / 1000) as i64;
        Option::Some(
            ValueHolder::LocalDateTime(NaiveDateTime::from_timestamp(seconds, nanos )))
    }

    fn from_timestamp_ms_f64(ts: f64) -> Option<ValueHolder> {
        let rounded_ts = ts.round();
        let seconds = rounded_ts as i64;
        let nanos = ((ts - rounded_ts) * 1_000_000_000f64) as u32;
        Option::Some(
            ValueHolder::LocalDateTime(NaiveDateTime::from_timestamp(seconds, nanos)))
    }

}

pub struct LocalDateExtractor;

impl ValueExtractor for LocalDateExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(val) => {
                return match val.parse::<NaiveDate>() {
                    Ok(date) => Result::Ok(ValueHolder::LocalDate(date)),
                    Err(_) => Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Strict, ParsingValueSource::String)),
                }
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        };
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(
            ValueExtractionError::PolicyNotSupported(
                ValueExtractionPolicy::Lax))
    }

}

pub struct LocalTimeExtractor;

impl ValueExtractor for LocalTimeExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match input.value {
            Value::String(val) => {
                return match val.parse::<NaiveTime>() {
                    Ok(time) => Result::Ok(ValueHolder::LocalTime(time)),
                    Err(_) => Result::Err(
                        ValueExtractionError::ParsingError(
                            ValueExtractionPolicy::Strict, ParsingValueSource::String)),
                }
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        };
    }

    fn lax_extract(_input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(
            ValueExtractionError::PolicyNotSupported(
                ValueExtractionPolicy::Lax))
    }

}