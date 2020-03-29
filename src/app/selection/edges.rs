use crate::app::values::ValuesPayload;

pub mod conditions;

pub trait SelectionEdge {

    fn get_id(&self) -> i32;
    fn get_next_selection_node_id(&self) -> i32;
    fn should_go_to_next(&self, payload: &ValuesPayload) -> bool;

}