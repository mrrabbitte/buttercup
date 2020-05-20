use crate::app::common::addressable::Address;
use crate::app::selection::edges::always::AlwaysTrueSelectionEdge;
use crate::app::selection::nodes::SelectionNodeAddress;
use crate::app::values::ValuesPayload;
use crate::app::selection::edges::logical::LogicalExpressionSelectionEdge;
use crate::app::selection::edges::logical::expressions::ExpressionEvaluationError;

use serde::{Serialize, Deserialize};

pub mod always;
pub mod logical;

#[derive(Serialize, Deserialize)]
pub enum SelectionEdge {

    AlwaysTrueSelectionEdge(AlwaysTrueSelectionEdge),
    LogicalExpressionSelectionEdge(LogicalExpressionSelectionEdge)

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionEdgeError {

    LogicalExpressionSelectionEdgeError(ExpressionEvaluationError)

}

pub trait SelectionEdgeDelegate {

    fn get_id(&self) -> &i32;

    fn get_next_selection_node(&self) -> &SelectionNodeAddress;

    fn can_pass(&self, payload: &ValuesPayload) -> Result<bool, SelectionEdgeError>;

    fn is_always_true(&self) -> bool {
        false
    }

    fn matches(&self, address: &SelectionEdgeAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

impl SelectionEdgeDelegate for SelectionEdge {

    fn get_id(&self) -> &i32 {
        self.get_delegate().get_id()
    }

    fn get_next_selection_node(&self) -> &SelectionNodeAddress {
        self.get_delegate().get_next_selection_node()
    }

    fn can_pass(&self, payload: &ValuesPayload) -> Result<bool, SelectionEdgeError> {
        self.get_delegate().can_pass(payload)
    }

}

impl SelectionEdge {

    fn get_delegate(&self) -> &dyn SelectionEdgeDelegate {
        return match self {
            SelectionEdge::AlwaysTrueSelectionEdge(edge) => edge,
            SelectionEdge::LogicalExpressionSelectionEdge(edge) => edge
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionEdgeAddress {

    id: i32,
    index: usize

}

impl Address for SelectionEdgeAddress {

    fn new(id: i32, index: usize) -> Self {
        SelectionEdgeAddress{
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

#[derive(Debug, Serialize, Deserialize)]
pub enum SelectionEdgeType {

    AlwaysTrueSelectionEdge,
    LogicalExpressionSelectionEdge

}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectionEdgeDefinition {

    id: i32,
    selection_node_definition_id: i32,
    selection_edge_type: SelectionEdgeType

}

impl SelectionEdgeDefinition {

    pub fn new(id: i32,
               selection_node_definition_id: i32,
               selection_edge_type: SelectionEdgeType) -> SelectionEdgeDefinition {
        SelectionEdgeDefinition {
            id,
            selection_node_definition_id,
            selection_edge_type
        }
    }

}
