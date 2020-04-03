use crate::app::values::ValuesPayload;

pub mod conditions;
pub mod operators;
pub mod expressions;

pub trait SelectionEdge {

    fn get_id(&self) -> i32;
    fn get_next_selection_node_id(&self) -> i32;
    fn can_pass(&self, payload: &ValuesPayload) -> bool;

}

pub struct SelectionEdgeDefinition {

    id: i32,
    selection_node_definition_id: i32

}

pub struct ExpressionSelectionEdgeDetails {

    selection_edge_definition_id: i32,
    
}
