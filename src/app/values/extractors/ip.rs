use std::net::IpAddr;
use std::str::FromStr;

use serde_json::Value;

use crate::app::values::extractors::{ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct IpAddressValueExtractor;

const INVALID_IP: &str = "Invalid IP provided.";

impl ValueExtractor for IpAddressValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::String(str_val) => match IpAddr::from_str(&str_val) {
                Ok(ip_addr) => Result::Ok(ValueHolder::IpAddress(ip_addr)),
                Err(_) => Result::Err(
                    ValueExtractionError::InvalidValueError(
                        ValueExtractionPolicy::Strict, INVALID_IP.to_string())),
            },
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(
                    ValueExtractionPolicy::Strict))
        }
    }

    fn lax_extract(_input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(ValueExtractionError::PolicyNotSupported(ValueExtractionPolicy::Lax))
    }

}