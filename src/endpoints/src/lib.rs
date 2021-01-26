use std::collections::HashMap;

use buttercup_values::{ValuesPayload, ValueType};
use buttercup_values::extractors::ValueExtractionPolicy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::extraction::{ArgumentsExtractionInput, ArgumentValueExtractorError, ArgumentValuesExtractionService};

pub mod extraction;

pub struct ArgumentSetDefinition {

    id: i32,
    name: String,
    description: String

}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ArgumentsExtractor {

    argument_definitions: HashMap<String, ArgumentDefinition>

}

impl ArgumentsExtractor {

    pub fn new(argument_definitions: HashMap<String, ArgumentDefinition>) -> ArgumentsExtractor {
        ArgumentsExtractor {
            argument_definitions
        }
    }

    pub fn extract(&self, payload: &Value) -> Result<ValuesPayload, ArgumentValueExtractorError> {
        ArgumentValuesExtractionService::process(
            ArgumentsExtractionInput::new(
                &self.argument_definitions,
                payload)
        )
    }

}