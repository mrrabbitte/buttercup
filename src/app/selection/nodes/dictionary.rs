use std::collections::HashMap;

use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::{SelectionNodeDefinition, SelectionNodeDelegate, SelectionNodeError};
use crate::app::values::{ValueHolder, ValuesPayload};

pub struct DictionarySelectionNodeDetails {

    selection_node_definition_id: i32,
    default_command_id: i32,
    target_value_name: String

}

impl DictionarySelectionNodeDetails {

    pub fn new(selection_node_definition_id: i32,
               default_command_id: i32,
               target_value_name: String) -> DictionarySelectionNodeDetails {
        DictionarySelectionNodeDetails {
            selection_node_definition_id,
            default_command_id,
            target_value_name
        }
    }

}

pub struct DictionarySelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edges: Vec<SelectionEdgeAddress>,
    details: DictionarySelectionNodeDetails,
    mapping: DictionaryNodeMapping

}

impl DictionarySelectionNode {

    pub fn new(definition: SelectionNodeDefinition,
               outgoing_edges: Vec<SelectionEdgeAddress>,
               details: DictionarySelectionNodeDetails,
               mapping: DictionaryNodeMapping) -> DictionarySelectionNode {
        DictionarySelectionNode {
            definition,
            outgoing_edges,
            details,
            mapping
        }
    }

}

impl SelectionNodeDelegate for DictionarySelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress> {
        &self.outgoing_edges
    }

    fn select_content_command_id(&self, payload: &ValuesPayload) -> Result<&i32, SelectionNodeError> {
        let target_value_name = &self.details.target_value_name;
        return match payload.get(&self.details.target_value_name) {
            None => Result::Err(
                SelectionNodeError::DictionarySelectionError(
                    DictionarySelectionError::ValueOfTargetNameNotFound(
                        target_value_name.clone()))),
            Some(value_holder) => self.mapping.get(value_holder)
        };
    }

}

pub enum DictionarySelectionError {

    ValueOfTargetNameNotFound(String)

}

pub struct DictionaryNodeMapping {

    default_command_id: i32,
    values: HashMap<ValueHolder, i32>

}

impl DictionaryNodeMapping {

    pub fn new(default_command_id: i32,
               values: HashMap<ValueHolder, i32>) -> DictionaryNodeMapping {
        DictionaryNodeMapping {
            default_command_id,
            values
        }
    }

    fn get(&self, key: &ValueHolder) -> Result<&i32, SelectionNodeError> {
        return match self.values.get(key) {
            None => Result::Ok(&self.default_command_id),
            Some(command_id) => Result::Ok(command_id),
        };
    }

}