use std::collections::HashMap;

use serde_json::{Map, Value};
use strum_macros::AsRefStr;

use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValueExtractorError, ArgumentValuesExtractionService};
use crate::app::values::{ValueHolder, ValuesPayload, ValueType};
use crate::app::values::extractors::{ValueExtractionPolicy, ValueExtractorInput, ValueExtractorService};

pub mod extraction;

pub struct ArgumentSetDefinition {

    id: i32,
    name: String,
    description: String

}

pub struct ArgumentDefinition {

    id: i32,
    name: String,
    argument_type: ValueType,
    extraction_policy: ValueExtractionPolicy,
    argument_set_definition_id: i32

}

impl ArgumentDefinition {

    pub fn new(id: i32,
               name: String,
               argument_type: ValueType,
               extraction_policy: ValueExtractionPolicy,
               argument_set_definition_id: i32) -> ArgumentDefinition {
        ArgumentDefinition {
            id,
            name,
            argument_type,
            extraction_policy,
            argument_set_definition_id
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_argument_type(&self) -> &ValueType {
        &self.argument_type
    }

    pub fn get_extraction_policy(&self) -> &ValueExtractionPolicy {
        &self.extraction_policy
    }

    pub fn get_argument_set_definition_id(&self) -> &i32 {
        &self.argument_set_definition_id
    }

}

pub struct ArgumentsExtractor {

    argument_definitions: HashMap<String, ArgumentDefinition>

}

impl ArgumentsExtractor {

    pub fn extract(&self, payload: &Value) -> Result<ValuesPayload, ArgumentValueExtractorError> {
        ArgumentValuesExtractionService::process(
            ArgumentsExtractionInput::new(
                &self.argument_definitions,
                payload)
        )
    }

}