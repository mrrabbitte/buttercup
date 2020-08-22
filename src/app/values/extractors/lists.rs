use crate::app::values::extractors::{ValueExtractor, ValueExtractionError, ValueExtractorInput, ValueExtractionPolicy, ValueExtractorService};
use crate::app::values::ValueHolder;
use serde_json::Value;
use crate::app::values::lists::{ValueHoldersList, ValueHoldersListError};

pub struct ListExtractor;

impl ValueExtractor for ListExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        match input.value {
            Value::Array(array) =>
                ListExtractor::do_extract(input, array),
            _ => Result::Err(
                ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Lax)),
        }
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        Result::Err(ValueExtractionError::PolicyNotSupported(ValueExtractionPolicy::Lax))
    }
}

impl ListExtractor {

    fn do_extract(parent_input: &ValueExtractorInput,
                  values: &Vec<Value>) -> Result<ValueHolder, ValueExtractionError> {
        let mut value_holders = Vec::new();
        for value in values {
            match ValueExtractorService::extract(
                &ValueExtractorInput::from_value(parent_input, value)) {
                Ok(value_holder) => value_holders.push(value_holder),
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        match ValueHoldersList::new(value_holders,
                                    parent_input.argument_type.clone()) {
            Ok(value_holders_list) =>
                Result::Ok(ValueHolder::List(value_holders_list)),
            Err(err) =>
                Result::Err(ValueExtractionError::ValueHoldersListError(err))
        }
    }
}