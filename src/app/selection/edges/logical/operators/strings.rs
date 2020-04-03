use crate::app::selection::edges::logical::operators::{RelationalOperator, RelationalOperatorError};
use crate::app::values::ValueHolder;

pub struct StringOperators;

impl StringOperators {

    pub fn handle(operator: &RelationalOperator,
                  left: &ValueHolder,
                  right: &ValueHolder) -> Result<bool, RelationalOperatorError> {
        return match left {
            ValueHolder::String(left_str) => match right {
                ValueHolder::String(right_str) =>
                    StringOperators::do_handle(operator, left_str, right_str),
                _ => Result::Err(
                    RelationalOperatorError::UnsupportedValueTypeForOperator(
                        right.clone(), operator.clone()))
            },
            _ => Result::Err(
                RelationalOperatorError::UnsupportedValueTypeForOperator(
                    left.clone(), operator.clone()))
        };
    }

    fn do_handle(operator: &RelationalOperator,
                 left_str: &String,
                 right_str: &String) -> Result<bool, RelationalOperatorError> {
        return match operator {
            RelationalOperator::Contains => Result::Ok(left_str.contains(right_str)),
            RelationalOperator::StartsWith => Result::Ok(left_str.starts_with(right_str)),
            RelationalOperator::EndsWith => Result::Ok(left_str.ends_with(right_str)),
            _ => Result::Err(
                RelationalOperatorError::UnsupportedOperatorForStrings(operator.clone()))
        };
    }

}