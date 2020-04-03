use crate::app::selection::nodes::{SelectionNodeDefinition, SelectionNodeDelegate, SelectionError};
use crate::app::values::ValuesPayload;

pub struct SimpleSelectionNodeDetails {

    selection_node_definition_id: i32,
    content_command_definition_id: i32

}

pub struct SimpleSelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edge_ids: Vec<i32>,
    details: SimpleSelectionNodeDetails

}

impl SelectionNodeDelegate for SimpleSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edge_ids(&self) -> &Vec<i32> {
        &self.outgoing_edge_ids
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload) -> Result<&i32, SelectionError> {
        Result::Ok(&self.details.content_command_definition_id)
    }

}