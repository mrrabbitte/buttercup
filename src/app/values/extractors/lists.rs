use serde_json::Value;

use crate::app::values::extractors::{ListExtractorInput, ValueExtractionError, ValueExtractionPolicy, ValueExtractor, ValueExtractorInput, ValueExtractorService};
use crate::app::values::lists::{ValueHoldersList, ValueHoldersListError};
use crate::app::values::ValueHolder;

pub struct ListExtractor;

impl ListExtractor {

    pub fn extract(input: &ListExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::Array(array) =>
                ListExtractor::do_extract(input, array),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Lax)),
        }
    }

    fn do_extract(input: &ListExtractorInput,
                  values: &Vec<Value>) -> Result<ValueHolder, ValueExtractionError> {
        let mut value_holders = Vec::new();
        for value in values {
            match ValueExtractorService::extract(
                &ValueExtractorInput::new(
                    value, input.elements_type, input.elements_policy)) {
                Ok(value_holder) => value_holders.push(value_holder),
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        match ValueHoldersList::new(value_holders,
                                    input.elements_type.clone()) {
            Ok(value_holders_list) =>
                Result::Ok(ValueHolder::List(value_holders_list)),
            Err(err) =>
                Result::Err(ValueExtractionError::ValueHoldersListError(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use super::*;

    #[test]
    fn test_simple_extraction() {

    }

}