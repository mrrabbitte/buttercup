use std::sync::Arc;

use serde_json::Value;

use crate::extractors::{ListExtractionError, ListExtractorInput, ValueExtractorInput, ValueExtractorService};
use crate::lists::ValueHoldersList;
use crate::ValueHolder;

pub struct ListExtractor;

impl ListExtractor {

    pub fn extract(input: &ListExtractorInput) -> Result<ValueHolder, ListExtractionError> {
        match input.value {
            Value::Array(array) =>
                ListExtractor::do_extract(input, array),
            _ => Result::Err(ListExtractionError::InvalidValueType),
        }
    }

    fn do_extract(input: &ListExtractorInput,
                  values: &Vec<Value>) -> Result<ValueHolder, ListExtractionError> {
        let mut value_holders = Vec::new();
        for (idx, value) in values.iter().enumerate() {
            match ValueExtractorService::extract(
                &ValueExtractorInput::new(
                    value, input.elements_type, input.elements_policy)) {
                Ok(value_holder) => value_holders.push(value_holder),
                Err(err) => {
                    return Result::Err(
                        ListExtractionError::ListElementExtractionError(err, idx));
                }
            }
        }
        match ValueHoldersList::new(value_holders,
                                    input.elements_type.clone()) {
            Ok(value_holders_list) =>
                Result::Ok(ValueHolder::List(Arc::new(value_holders_list))),
            Err(err) =>
                Result::Err(ListExtractionError::ValueHoldersListError(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::extractors::{ValueExtractionError, ValueExtractionPolicy};
    use crate::ValueType;

    use super::*;

    const ERROR_ELEMENT_TYPE_LIST: &str = "[0, 1.23, 2]";
    const EMPTY_LIST: &str = "[]";
    const SIMPLE_LIST: &str = "[0, 1, 2, 3, 4]";
    const STRING_LIST: &str = r#"["a", "b", "c"]"#;

    #[test]
    fn test_simple_extraction() {
        extract_and_assert_condition(SIMPLE_LIST,
                                     |list| list.get_elements().len() == 5);
        let result =
            extract(STRING_LIST, &ValueType::String);

        assert!(result.is_ok());
        assert!(
            matches!(
            result.unwrap(),
            ValueHolder::List(list)
            if list.get_elements().len() == 3
            && list.get_elements().iter().filter_map(|element| match element {
                ValueHolder::String(val) => Some(val),
                _ => None
            }).count() == 3));
    }

    #[test]
    fn test_empty_list_extraction() {
        extract_and_assert_condition(EMPTY_LIST,
                                     |list| list.get_elements().is_empty());
    }

    #[test]
    fn test_mismatched_element_type_error() {
        extract_and_assert_error(
            ERROR_ELEMENT_TYPE_LIST,
            ListExtractionError::ListElementExtractionError(
                ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Strict), 1));
    }

    #[test]
    fn test_mismatched_list_type_error() {
        let result =
            extract(STRING_LIST, &ValueType::Integer);

        assert!(result.is_err(), "{:?}", result);

        assert_eq!(ListExtractionError::ListElementExtractionError(
            ValueExtractionError::InvalidValueTypeError(ValueExtractionPolicy::Strict), 0),
                   result.unwrap_err());
    }

    fn extract(input: &str, value_type: &ValueType) -> Result<ValueHolder, ListExtractionError> {
        let value: Value = serde_json::from_str(input).unwrap();

        ListExtractor::extract(
            &ListExtractorInput::new(
                &value,
                &value_type,
                &ValueExtractionPolicy::Strict))
    }

    fn extract_integer(input: &str) -> Result<ValueHolder, ListExtractionError> {
        extract(input, &ValueType::Integer)
    }

    fn extract_and_assert_condition<F>(input: &str,
                                       condition: F) where F: Fn(&ValueHoldersList) -> bool {
        let result = extract_integer(input);

        assert!(result.is_ok(), "{:?}", result);

        let value_holder = result.unwrap();

        assert!(
            matches!(
            value_holder,
            ValueHolder::List(list) if condition(&list))
        );
    }

    fn extract_and_assert_error(input: &str,
                                expected: ListExtractionError) {
        let result = extract_integer(input);

        assert!(result.is_err(), "{:?}", result);

        assert_eq!(expected, result.unwrap_err());
    }
}