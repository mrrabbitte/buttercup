use crate::app::selection::addressable::Address;
use crate::app::selection::edges::logical::conditions::{Condition, ConditionEvaluationError};
use crate::app::selection::edges::logical::operators::LogicalOperator;
use crate::app::values::ValuesPayload;

pub struct ExpressionDefinition {

    id: i32,
    internal_logical_operator: LogicalOperator

}

pub struct Expression {

    definition: ExpressionDefinition,
    conditions: Vec<Condition>,
    next_expression_address: Option<ExpressionAddress>,
    operator_between_expressions: LogicalOperator

}

#[derive(Copy, Clone, Debug)]
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
            Ok(result) => match self.next_expression_address {
                None => Result::Ok(result),
                Some(address) =>
                    match expressions.get(address.index) {
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
                                        match self.operator_between_expressions {
                                            LogicalOperator::And =>
                                                Result::Ok(result && next_expression_result),
                                            LogicalOperator::Or =>
                                                Result::Ok(result || next_expression_result),
                                        },
                                    Err(error) =>
                                        Result::Err(error),
                                };
                            },
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

    fn matches(&self, address: ExpressionAddress) -> bool {
        self.definition.id == *address.get_id()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {

    }

}