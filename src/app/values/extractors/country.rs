use isocountry::{CountryCode, CountryCodeParseErr};
use serde_json::Value;

use crate::app::values::extractors::{CountryCodeParsingError, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct CountryValueExtractor;

impl ValueExtractor for CountryValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match CountryCode::for_alpha3_caseless(str_val) {
                Ok(country_code) => Result::Ok(ValueHolder::Country(country_code)),
                Err(err) =>
                    Result::Err(
                        ValueExtractionError::CountryCodeParsingError(
                            ValueExtractionPolicy::Strict, CountryCodeParsingError::from(err))),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match CountryCode::for_alpha2_caseless(str_val) {
                Ok(country_code) => Result::Ok(ValueHolder::Country(country_code)),
                Err(err) =>
                    Result::Err(
                        ValueExtractionError::CountryCodeParsingError(
                            ValueExtractionPolicy::Lax, CountryCodeParsingError::from(err))),
            },
            Value::Number(num) => {
                if !num.is_u64() && !num.is_i64() {
                    return Result::Err(
                        ValueExtractionError::InvalidValueTypeError(
                            ValueExtractionPolicy::Lax));
                }
                match num.as_u64() {
                    None => Result::Err(
                        ValueExtractionError::InvalidValueTypeError(
                            ValueExtractionPolicy::Lax)),
                    Some(num_val) => match CountryCode::for_id(num_val as u32) {
                        Ok(country_code) =>
                            Result::Ok(ValueHolder::Country(country_code)),
                        Err(err) => Result::Err(
                            ValueExtractionError::CountryCodeParsingError(
                                ValueExtractionPolicy::Lax, CountryCodeParsingError::from(err))),
                    },
                }
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Lax))
        }
    }

}