use crate::app::selection::nodes::{SelectionNodeDefinition, SelectionNodeDelegate};
use std::collections::HashMap;
use crate::app::values::{ValueHolder, ValuesPayload};

pub struct DictionarySelectionNodeDetails {

    selection_node_definition_id: i32,
    default_command_id: i32,
    target_value_name: String

}

pub struct DictionarySelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edge_ids: Vec<i32>,
    details: DictionarySelectionNodeDetails,
    mapping: HashMap<ValueHolder, i32>

}

impl SelectionNodeDelegate for DictionarySelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edge_ids(&self) -> &Vec<i32> {
        &self.outgoing_edge_ids
    }

    fn select_content_command_id(&self, payload: &ValuesPayload) -> &i32 {

    }

}