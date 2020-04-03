use crate::app::values::ValuesPayload;
use crate::app::selection::nodes::SelectionNodeAddress;
use crate::app::selection::edges::always::AlwaysTrueSelectionEdge;

pub mod always;
pub mod logical;

pub enum SelectionEdge {

    AlwaysTrueSelectionEdge(AlwaysTrueSelectionEdge),
    LogicalExpressionSelectionEdge

}

pub trait SelectionEdgeDelegate {

    fn get_id(&self) -> &i32;
    fn get_next_selection_node(&self) -> &SelectionNodeAddress;
    fn can_pass(&self, payload: &ValuesPayload) -> bool;
    fn is_always_true(&self) -> bool {
        false
    }

}

pub struct SelectionEdgeAddress {

    id: i32,
    idx: i32

}

pub struct SelectionEdgeDefinition {

    id: i32,
    selection_node_definition_id: i32

}

pub struct ExpressionSelectionEdgeDetails {

    selection_edge_definition_id: i32,

}
