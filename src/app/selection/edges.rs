use crate::app::values::ValuesPayload;

pub mod conditions;
pub mod operators;

pub trait SelectionEdge {

    fn get_id(&self) -> i32;
    fn get_next_selection_node_id(&self) -> i32;
    fn can_pass(&self, payload: &ValuesPayload) -> bool;

}