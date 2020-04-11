use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::{SelectionNodeError, SelectionNodeDefinition, SelectionNodeDelegate};
use crate::app::values::ValuesPayload;

#[derive(Debug)]
pub struct SimpleSelectionNodeDetails {

    selection_node_definition_id: i32,
    content_command_definition_id: i32

}

impl SimpleSelectionNodeDetails {

    pub fn new(selection_node_definition_id: i32,
               content_command_definition_id: i32) -> SimpleSelectionNodeDetails {
        SimpleSelectionNodeDetails {
            selection_node_definition_id,
            content_command_definition_id
        }
    }

}

#[derive(Debug)]
pub struct SimpleSelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edges: Vec<SelectionEdgeAddress>,
    details: SimpleSelectionNodeDetails

}

impl SimpleSelectionNode {

    pub fn new(definition: SelectionNodeDefinition,
               outgoing_edges: Vec<SelectionEdgeAddress>,
               details: SimpleSelectionNodeDetails) -> SimpleSelectionNode {
        SimpleSelectionNode {
            definition,
            outgoing_edges,
            details
        }
    }

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