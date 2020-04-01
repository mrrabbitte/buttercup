use std::collections::HashMap;

use predicates::ord::le;
use serde_json::Value;

use crate::app::selection::edges::operators::{RelationalOperator, RelationalOperatorError};
use crate::app::selection::SelectionEvaluationError;
use crate::app::values::{ValueHolder, ValuesPayload, ValueType};

pub struct ConditionDefinition {

    id: i32,
    left_value_name: String,
    operator_type: RelationalOperator,
    is_negation: bool,

}

pub struct StaticValueConditionDefinition {

    condition_definition_id: i32,
    right_value: Value,
    right_value_type: ValueType

}

pub struct RuntimeValueConditionDefinition {

    condition_definition_id: i32,
    right_value_name: String

}

pub struct Condition {

    id: i32,
    left_value_name: String,
    operator: RelationalOperator,
    is_negation: bool,
    value: ConditionValue

}

pub enum ConditionValue {

    Static(ValueHolder),
    Runtime(String)

}

pub enum ConditionEvaluationError {

    DidNotFindLeftValue(String),
    DidNotFindRightValue(String),
    OperatorEvaluationError(RelationalOperatorError)

}

impl Condition {

    pub fn evaluate(&self,
                    payload: &ValuesPayload) -> Result<bool, ConditionEvaluationError> {
        let left_value_name = &self.left_value_name;
        return match payload.get(left_value_name){
            Some(left_value) => self.handle(left_value, payload),
            None => Result::Err(
                ConditionEvaluationError::DidNotFindLeftValue(left_value_name.clone())),
        }
    }

    fn handle(&self,
              left_value: &ValueHolder,
              payload: &ValuesPayload) -> Result<bool, ConditionEvaluationError> {
        return match &self.value {
            ConditionValue::Static(right_value) =>
                self.handle_evaluation(left_value, right_value),
            ConditionValue::Runtime(right_value_name) =>
                match payload.get(right_value_name) {
                    Some(right_value) =>
                        self.handle_evaluation(left_value, right_value),
                    None => Result::Err(
                        ConditionEvaluationError::DidNotFindRightValue(
                            right_value_name.clone())),
            },
        };
    }

    fn handle_evaluation(&self,
                         left_value: &ValueHolder,
                         right_value: &ValueHolder) -> Result<bool, ConditionEvaluationError> {
        return match &self.operator.evaluate(left_value, right_value) {
            Ok(result) => Result::Ok(*result),
            Err(error) =>
                Result::Err(
                    ConditionEvaluationError::OperatorEvaluationError(error.clone())),
        }
    }

}