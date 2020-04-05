use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::{SelectionNodeError, SelectionNodeDefinition, SelectionNodeDelegate};
use crate::app::values::ValuesPayload;

pub struct SimpleSelectionNodeDetails {

    selection_node_definition_id: i32,
    content_command_definition_id: i32

}

pub struct SimpleSelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edges: Vec<SelectionEdgeAddress>,
    details: SimpleSelectionNodeDetails

}

impl SelectionNodeDelegate for SimpleSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress> {
        &self.outgoing_edges
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload) -> Result<&i32, SelectionNodeError> {
        Result::Ok(&self.details.content_command_definition_id)
    }

}