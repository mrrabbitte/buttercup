use std::iter::Map;

use serde_json::Value;
use strum_macros::AsRefStr;

use crate::arguments::description::ArgumentDescription;
use crate::arguments::values::ValueHolder;

pub mod extractors;
pub mod values;
pub mod description;

pub struct ArgumentsProcessorInput {

    argument_descriptions: Vec<ArgumentDescription>,
    payload: Value

}

pub struct ArgumentValuesExtractor;

impl ArgumentValuesExtractor {

    pub fn process(input: ArgumentsProcessorInput) -> Map<String, ValueHolder> {
        
    }

}