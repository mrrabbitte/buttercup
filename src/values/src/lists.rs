use crate::{ValueHolder, ValueType};

use strum_macros::{AsRefStr, EnumVariantNames};
use serde::{Deserialize, Serialize};

#[derive(AsRefStr, EnumVariantNames,
    Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ValueHoldersListError {

    FoundElementOfUnexpectedValueType

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct ValueHoldersList {

    elements: Vec<ValueHolder>,
    value_type: ValueType

}

impl ValueHoldersList {

    pub fn new(elements: Vec<ValueHolder>,
               value_type: ValueType) -> Result<ValueHoldersList, ValueHoldersListError> {
        for element in &elements {
            if !value_type.matches(&element) {
                return Result::Err(ValueHoldersListError::FoundElementOfUnexpectedValueType);
            }
        }
        Result::Ok(
            ValueHoldersList {
                elements,
                value_type
            }
        )
    }

    pub fn get_elements(&self) -> &Vec<ValueHolder> {
        &self.elements
    }

    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use num::BigInt;

    #[test]
    fn test_builds_list_of_elements_correctly() {
        let mut elements = Vec::new();
        elements.push(ValueHolder::Boolean(true));
        elements.push(ValueHolder::Boolean(false));
        elements.push(ValueHolder::Boolean(true));
        let list =
            ValueHoldersList::new(elements.clone(), ValueType::Boolean).unwrap();
        assert_eq!(&elements, list.get_elements());
        assert_eq!(&ValueType::Boolean, list.get_value_type());
    }

    #[test]
    fn test_returns_error_on_mismatched_types() {
        let mut elements = Vec::new();
        elements.push(ValueHolder::Boolean(true));
        elements.push(ValueHolder::Boolean(false));
        elements.push(ValueHolder::Integer(BigInt::from(1)));
        let result =
            ValueHoldersList::new(elements.clone(), ValueType::Boolean);
        assert_eq!(true, result.is_err());
    }
}