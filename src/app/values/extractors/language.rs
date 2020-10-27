use isolang::Language;
use serde_json::Value;

use crate::app::values::extractors::{ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;
use crate::app::values::wrappers::{LanguageWrapper, Wrapper};

pub struct LanguageValueExtractor;

const INVALID_STRICT_MSG: &str = "Expected language code in iso-639-3 format.";

const INVALID_LAX_MSG: &str =
    "Expected language code in iso-639-3, iso-639-1 or locale string (e.g. 'de_DE.UTF-8') format.";

impl ValueExtractor for LanguageValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match Language::from_639_3(&str_val) {
                None => Result::Err(
                    ValueExtractionError::InvalidValueError(
                        ValueExtractionPolicy::Strict, INVALID_STRICT_MSG.to_string())),
                Some(language) =>
                    Result::Ok(ValueHolder::Language(LanguageWrapper::new(language))),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match Language::from_639_1(&str_val) {
                None => match Language::from_locale(&str_val) {
                    None => Result::Err(
                        ValueExtractionError::InvalidValueError(
                            ValueExtractionPolicy::Lax, INVALID_LAX_MSG.to_string())),
                    Some(language) =>
                        Result::Ok(ValueHolder::Language(LanguageWrapper::new(language)))
                },
                Some(language) =>
                    Result::Ok(ValueHolder::Language(LanguageWrapper::new(language))),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

}