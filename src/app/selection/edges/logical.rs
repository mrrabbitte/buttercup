use crate::app::selection::edges::{SelectionEdgeDefinition, SelectionEdgeDelegate, SelectionEdgeError};
use crate::app::selection::edges::logical::expressions::{Expression, ExpressionEvaluationError};
use crate::app::selection::nodes::SelectionNodeAddress;
use crate::app::values::ValuesPayload;

use serde::{Serialize, Deserialize};

pub mod conditions;
pub mod expressions;
pub mod operators;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogicalExpressionSelectionEdgeDetails {

    id: i32,
    selection_edge_definition_id: i32

}

impl LogicalExpressionSelectionEdgeDetails {

    pub fn new(id: i32,
               selection_edge_definition_id: i32) -> LogicalExpressionSelectionEdgeDetails {
        LogicalExpressionSelectionEdgeDetails {
            id,
            selection_edge_definition_id
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogicalExpressionSelectionEdge {

    definition: SelectionEdgeDefinition,
    next_selection_node: SelectionNodeAddress,
    details: LogicalExpressionSelectionEdgeDetails,
    expressions: Vec<Expression>,
    first_expression: Expression

}

impl LogicalExpressionSelectionEdge {

    pub fn new(definition: SelectionEdgeDefinition,
               next_selection_node: SelectionNodeAddress,
               details: LogicalExpressionSelectionEdgeDetails,
               expressions: Vec<Expression>,
               first_expression: Expression) -> LogicalExpressionSelectionEdge {
        LogicalExpressionSelectionEdge {
            definition,
            next_selection_node,
            details,
            expressions,
            first_expression
        }
    }

}

impl SelectionEdgeDelegate for LogicalExpressionSelectionEdge {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_next_selection_node(&self) -> &SelectionNodeAddress {
        &self.next_selection_node
    }

    fn can_pass(&self, payload: &ValuesPayload) -> Result<bool, SelectionEdgeError> {
        return match self.first_expression.evaluate(&self.expressions, payload) {
            Ok(result) => Result::Ok(result),
            Err(error) =>
                Result::Err(SelectionEdgeError::LogicalExpressionSelectionEdgeError(error)),
        };
    }

}

