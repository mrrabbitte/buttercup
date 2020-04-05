use std::collections::HashMap;

use crate::app::selection::nodes::{SelectionNodeError, SelectionNodeDefinition, SelectionNodeDelegate};
use crate::app::values::{ValueHolder, ValuesPayload};
use crate::app::selection::edges::SelectionEdgeAddress;

pub struct DictionarySelectionNodeDetails {

    selection_node_definition_id: i32,
    default_command_id: i32,
    target_value_name: String

}

pub struct DictionarySelectionNode {

    definition: SelectionNodeDefinition,
    outgoing_edges: Vec<SelectionEdgeAddress>,
    details: DictionarySelectionNodeDetails,
    mapping: DictionaryNodeMapping

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
            Some(value_holder) => match value_holder {
                _ => Result::Err(
                    SelectionNodeError::DictionarySelectionError(
                        DictionarySelectionError::ValueIsNotString(
                            target_value_name.clone()))),
                ValueHolder::String(key) => self.mapping.get(key)
            }
        };
    }

}

pub enum DictionarySelectionError {

    ValueOfTargetNameNotFound(String),
    MappingForValueNotFound(String),
    ValueIsNotString(String),

}

struct DictionaryNodeMapping {

    default_command_id: i32,
    values: HashMap<String, i32>

}

impl DictionaryNodeMapping {

    fn get(&self, key: &String) -> Result<&i32, SelectionNodeError> {
        return match &self.values.get(key) {
            None => Result::Err(
                SelectionNodeError::DictionarySelectionError(
                    DictionarySelectionError::MappingForValueNotFound(
                        key.clone()))),
            Some(command_id) => Result::Ok(command_id),
        };
    }

}