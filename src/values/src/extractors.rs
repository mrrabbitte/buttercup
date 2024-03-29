use isocountry::CountryCodeParseErr;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ValueHolder, ValueType};
use crate::extractors::boolean::BooleanExtractor;
use crate::extractors::country::CountryValueExtractor;
use crate::extractors::date_time::day_of_week::DayOfWeekExtractor;
use crate::extractors::date_time::local::{LocalDateExtractor, LocalDateTimeExtractor, LocalTimeExtractor};
use crate::extractors::date_time::zoned::{TimezoneExtractor, ZonedDateTimeExtractor};
use crate::extractors::email::EmailValueExtractor;
use crate::extractors::geolocation::GeoCoordinatesExtractor;
use crate::extractors::ip::IpAddressValueExtractor;
use crate::extractors::language::LanguageValueExtractor;
use crate::extractors::lists::ListExtractor;
use crate::extractors::number::{DecimalExtractor, IntegerExtractor};
use crate::extractors::string::StringExtractor;
use crate::geolocation::GeoCoordinatesValueError;
use crate::lists::ValueHoldersListError;
use crate::zoned_date_time::ZonedDateTimeParsingError;
use crate::extractors::date_time::duration::DurationExtractor;

pub(crate) mod boolean;
pub(crate) mod country;
pub(crate) mod date_time;
pub(crate) mod email;
pub(crate) mod geolocation;
pub(crate) mod ip;
pub(crate) mod language;
pub(crate) mod lists;
pub(crate) mod number;
pub(crate) mod string;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueExtractionPolicy {

    Strict,
    Lax

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueExtractionError {

    InvalidValueTypeError(ValueExtractionPolicy),
    InvalidValueError(ValueExtractionPolicy, String),
    ParsingError(ValueExtractionPolicy, ParsingValueSource),
    GeoCoordinatesValueError(ValueExtractionPolicy, GeoCoordinatesValueError),
    JsonDeserializationError(ValueExtractionPolicy, String),
    PolicyNotSupported(ValueExtractionPolicy),
    ZonedDateTimeParsingError(ValueExtractionPolicy, ZonedDateTimeParsingError),
    CountryCodeParsingError(ValueExtractionPolicy, CountryCodeParsingError),
    EmailParsingError(ValueExtractionPolicy, String),
    InvalidInputTypeForList,
    ValueIsNull

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListExtractionError {

    ValueIsNull,
    InvalidValueType,
    InvalidInputTypeForList,
    ListElementExtractionError(ValueExtractionError, usize),
    ValueHoldersListError(ValueHoldersListError)

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParsingValueSource {

    String,
    I64,
    U64,
    F64

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CountryCodeParsingError {

    InvalidAlpha2,
    InvalidAlpha3,
    InvalidCountryId

}

impl From<CountryCodeParseErr> for CountryCodeParsingError {

    fn from(err: CountryCodeParseErr) -> Self {
        match err {
            CountryCodeParseErr::InvalidAlpha2 {..} => CountryCodeParsingError::InvalidAlpha2,
            CountryCodeParseErr::InvalidAlpha3 {..} => CountryCodeParsingError::InvalidAlpha3,
            CountryCodeParseErr::InvalidID {..} => CountryCodeParsingError::InvalidCountryId,
        }
    }
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

#[derive(Debug)]
pub struct ListExtractorInput<'a> {

    value: &'a Value,
    elements_type: &'a ValueType,
    elements_policy: &'a ValueExtractionPolicy

}

impl<'a> ListExtractorInput<'a> {

    pub fn new(value: &'a Value,
               elements_type: &'a ValueType,
               elements_policy: &'a ValueExtractionPolicy) -> ListExtractorInput<'a> {
        ListExtractorInput {
            value,
            elements_type,
            elements_policy
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
            ValueType::GeoCoordinates => GeoCoordinatesExtractor::extract(input),
            ValueType::Language => LanguageValueExtractor::extract(input),
            ValueType::Country => CountryValueExtractor::extract(input),
            ValueType::Email => EmailValueExtractor::extract(input),
            ValueType::IpAddress => IpAddressValueExtractor::extract(input),
            ValueType::Duration => DurationExtractor::extract(input),
            ValueType::List => Result::Err(ValueExtractionError::InvalidInputTypeForList),
        };
    }

    pub fn extract_list(input: &ListExtractorInput) -> Result<ValueHolder, ListExtractionError> {
        if input.value.is_null() {
            return Result::Err(ListExtractionError::ValueIsNull);
        }
        return ListExtractor::extract(input);
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

