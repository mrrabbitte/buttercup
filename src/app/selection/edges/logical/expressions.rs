use crate::app::common::addressable::Address;
use crate::app::selection::edges::logical::conditions::{Condition, ConditionEvaluationError};
use crate::app::selection::edges::logical::operators::LogicalOperator;
use crate::app::values::ValuesPayload;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionDefinition {

    id: i32,
    internal_logical_operator: LogicalOperator

}

impl ExpressionDefinition {

    pub fn new(id: i32,
               internal_logical_operator: LogicalOperator) -> ExpressionDefinition {
        ExpressionDefinition {
            id,
            internal_logical_operator
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expression {

    definition: ExpressionDefinition,
    conditions: Vec<Condition>,
    next_expression: Option<NextExpressionAddressWithOperator>

}

impl Expression {

    pub fn new(definition: ExpressionDefinition,
               conditions: Vec<Condition>,
               next_expression: Option<NextExpressionAddressWithOperator>) -> Expression {
        Expression {
            definition,
            conditions,
            next_expression
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextExpressionAddressWithOperator {

    address: ExpressionAddress,
    operator_between_expressions: LogicalOperator

}

impl NextExpressionAddressWithOperator {

    pub fn new(address: ExpressionAddress,
               operator_between_expressions: LogicalOperator) -> NextExpressionAddressWithOperator {
        NextExpressionAddressWithOperator {
            address,
            operator_between_expressions
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionAddress {

    id: i32,
    index: usize

}

impl Address for ExpressionAddress {

    fn new(id: i32, index: usize) -> Self {
        ExpressionAddress {
            id,
            index
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }

    fn get_index(&self) -> &usize {
        &self.index
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpressionEvaluationError {

    MissingExpression(ExpressionAddress),
    ExpressionAddressIdMismatch(ExpressionAddress),
    ConditionEvaluationError(ConditionEvaluationError)

}

impl Expression {

    pub fn evaluate(&self,
                    expressions: &Vec<Expression>,
                    payload: &ValuesPayload) -> Result<bool, ExpressionEvaluationError> {
        return match self.evaluate_conditions(payload) {
            Ok(result) => match &self.next_expression {
                None => Result::Ok(result),
                Some(address_with_operator) => {
                    let address = &address_with_operator.address;
                    match expressions.get(address_with_operator.address.index) {
                        None => Result::Err(
                            ExpressionEvaluationError::MissingExpression(address.clone())),
                        Some(next_expression) =>
                            {
                                if !next_expression.matches(address) {
                                    return Result::Err(
                                        ExpressionEvaluationError::ExpressionAddressIdMismatch(
                                            address.clone()));
                                }
                                return match next_expression.evaluate(expressions, payload) {
                                    Ok(next_expression_result) =>
                                        match address_with_operator.operator_between_expressions {
                                            LogicalOperator::And =>
                                                Result::Ok(result && next_expression_result),
                                            LogicalOperator::Or =>
                                                Result::Ok(result || next_expression_result),
                                        },
                                    Err(error) =>
                                        Result::Err(error),
                                };
                            },
                    }
                },
            },
            Err(error) =>
                Result::Err(ExpressionEvaluationError::ConditionEvaluationError(error)),
        };
    }

    fn evaluate_conditions(&self,
                           payload: &ValuesPayload) -> Result<bool, ConditionEvaluationError> {
        let operator = &self.definition.internal_logical_operator;
        for condition in &self.conditions {
            match condition.evaluate(payload) {
                Ok(condition_result) => {
                    match operator {
                        LogicalOperator::And => {
                            if !condition_result {
                                return Result::Ok(false);
                            }
                        },
                        LogicalOperator::Or => {
                            if condition_result {
                                return Result::Ok(true);
                            }
                        },
                    };
                },
                Err(err) => return Result::Err(err),
            }
        }
        return match operator {
            LogicalOperator::And => Result::Ok(true),
            LogicalOperator::Or => Result::Ok(false),
        };
    }

    fn matches(&self, address: &ExpressionAddress) -> bool {
        self.definition.id == *address.get_id()
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::app::selection::edges::logical::conditions::ConditionValue;
    use crate::app::selection::edges::logical::operators::RelationalOperator;
    use crate::app::values::ValueHolder;

    use super::*;

    const VALUE_NAME: &str = "valueName";

    #[test]
    fn test_simple_ok() {
        let expression = simple_expression();
        let result = expression.evaluate(&vec![],
                            &ValuesPayload::new(
                                build_map(
                                    vec![(VALUE_NAME.to_string(),
                                    ValueHolder::String("Yata-man".to_string()))]
                                )));
        assert_eq!(true, result.is_ok());
        assert_eq!(true, result.unwrap());
    }

    #[test]
    fn test_simple_err() {
        let expression = simple_expression();
        let result = expression.evaluate(&vec![],
                                         &ValuesPayload::new(
                                             build_map(
                                                 vec![("WrongName".to_string(),
                                                       ValueHolder::String("Yata-man".to_string()))]
                                             )));
        assert_eq!(true, result.is_err());
    }

    fn simple_expression() -> Expression {
        Expression::new(
            ExpressionDefinition::new(
                2,  LogicalOperator::And),
            vec![
                Condition::new(5,
                               VALUE_NAME.to_string(),
                               RelationalOperator::StartsWith,
                               false,
                               ConditionValue::Static(
                                   ValueHolder::String(String::from("Yata"))
                               ))
            ],
            Option::None
        )
    }

    fn build_map<K,V>(entries: Vec<(K, V)>) -> HashMap<K, V>
        where K: Hash + Eq {
        let mut ret = HashMap::new();
        for entry in entries {
            ret.insert(entry.0, entry.1);
        }
        ret
    }

}