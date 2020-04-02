use std::collections::HashMap;

use serde_json::{Map, Value};
use strum_macros::AsRefStr;

use crate::app::values::{ValueHolder, ValuesPayload, ValueType};
use crate::app::values::extractors::{ValueExtractionPolicy, ValueExtractorInput, ValueExtractorService};

pub mod extraction;

pub struct ArgumentDefinition {

    id: i32,
    name: String,
    argument_type: ValueType,
    extraction_policy: ValueExtractionPolicy

}

impl ArgumentDefinition {

    pub fn new(id: i32,
               name: String,
               argument_type: ValueType,
               extraction_policy: ValueExtractionPolicy) -> ArgumentDefinition {
        ArgumentDefinition {
            id,
            name,
            argument_type,
            extraction_policy
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

}