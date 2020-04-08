use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::app::values::{ValueHolder, ValueType};
use crate::app::values::extractors::boolean::BooleanExtractor;
use crate::app::values::extractors::date_time::day_of_week::DayOfWeekExtractor;
use crate::app::values::extractors::date_time::local::{LocalDateExtractor, LocalDateTimeExtractor, LocalTimeExtractor};
use crate::app::values::extractors::date_time::zoned::{TimezoneExtractor, ZonedDateTimeExtractor};
use crate::app::values::extractors::geolocation::GeoCoordinatesExtractor;
use crate::app::values::extractors::number::{DecimalExtractor, IntegerExtractor};
use crate::app::values::extractors::string::StringExtractor;
use crate::app::values::zoned_date_time::ZonedDateTimeParsingError;

pub mod boolean;
pub mod date_time;
pub mod geolocation;
pub mod number;
pub mod string;

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueExtractionPolicy {

    Strict,
    Lax

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueExtractionError {

    InvalidValueTypeError(ValueExtractionPolicy),
    InvalidValueError(ValueExtractionPolicy, String),
    ParsingError(ValueExtractionPolicy, ParsingValueSource),
    PolicyNotSupported(ValueExtractionPolicy),
    ZonedDateTimeParsingError(ValueExtractionPolicy, ZonedDateTimeParsingError),
    ValueIsNull

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParsingValueSource {

    String,
    I64,
    U64,
    F64,
    Json

}

#[derive(Debug)]
pub struct ValueExtractorInput<'a> {

    value: &'a Value,
    argument_type: &'a ValueType,
    policy: &'a ValueExtractionPolicy

}

impl<'a> ValueExtractorInput<'a> {

    pub fn new(value: &'a Value,
               argument_type: &'a ValueType,
               policy: &'a ValueExtractionPolicy) -> ValueExtractorInput<'a> {
        ValueExtractorInput {
            value,
            argument_type,
            policy
        }
    }

}

pub struct ValueExtractorService;

impl ValueExtractorService {

    pub fn extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        if input.value.is_null() {
            return Result::Err(ValueExtractionError::ValueIsNull);
        }
        return match &input.argument_type {
            ValueType::Boolean => BooleanExtractor::extract(input),
            ValueType::String => StringExtractor::extract(input),
            ValueType::Decimal => DecimalExtractor::extract(input),
            ValueType::Integer => IntegerExtractor::extract(input),
            ValueType::LocalDateTime => LocalDateTimeExtractor::extract(input),
            ValueType::LocalDate => LocalDateExtractor::extract(input),
            ValueType::LocalTime => LocalTimeExtractor::extract(input),
            ValueType::DayOfWeek => DayOfWeekExtractor::extract(input),
            ValueType::TimeZone => TimezoneExtractor::extract(input),
            ValueType::ZonedDateTime => ZonedDateTimeExtractor::extract(input),
            ValueType::GeoCoordinates => GeoCoordinatesExtractor::extract(input)
        };
    }

}

pub trait ValueExtractor {

    fn extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        return match &input.policy {
            ValueExtractionPolicy::Strict => Self::strict_extract(input),
            ValueExtractionPolicy::Lax => {
                let strict_result = Self::strict_extract(input);
                if strict_result.is_ok() {
                    return strict_result;
                }
                return Self::lax_extract(input);
            },
        }
    }

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError>;
    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError>;

}

