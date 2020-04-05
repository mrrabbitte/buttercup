use crate::app::selection::edges::{SelectionEdgeDelegate, SelectionEdgeDefinition, SelectionEdgeError};
use crate::app::values::ValuesPayload;
use crate::app::selection::nodes::SelectionNodeAddress;

pub mod conditions;
pub mod expressions;
pub mod operators;

pub struct LogicalExpressionSelectionEdgeDetails {

    id: i32

}

pub struct LogicalExpressionSelectionEdge {

    definition: SelectionEdgeDefinition,
    next_selection_node: SelectionNodeAddress,
    details: LogicalExpressionSelectionEdgeDetails

}

impl SelectionEdgeDelegate for LogicalExpressionSelectionEdge {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_next_selection_node(&self) -> &SelectionNodeAddress {
        &self.next_selection_node
    }

    fn can_pass(&self, payload: &ValuesPayload) -> Result<bool, SelectionEdgeError> {
        unimplemented!()
    }

}

