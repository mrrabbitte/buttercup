use std::collections::HashMap;

use serde_json::{Map, Value};
use strum_macros::AsRefStr;

use crate::arguments::definition::ArgumentDefinition;
use crate::arguments::extractors::{ArgumentValueExtractor, ValueExtractionPolicy, ValueExtractorInput};
use crate::values::{ValueHolder, ValuesPayload};

pub mod extractors;
pub mod definition;

pub struct ArgumentsProcessorInput<'a> {

    definitions: HashMap<String, ArgumentDefinition>,
    payload: &'a Value

}

impl ArgumentsProcessorInput<'_> {

    pub fn new(definitions: HashMap<String, ArgumentDefinition>,
               payload: &Value) -> ArgumentsProcessorInput {
        ArgumentsProcessorInput {
            definitions,
            payload
        }
    }

}

#[derive(Debug)]
pub enum ArgumentValueExtractorError {

    MissingArgument(String),
    ExtractionFailure(String, Value, ValueExtractionPolicy),
    InvalidJsonInput

}

pub struct ArgumentValuesExtractor;

impl ArgumentValuesExtractor {

    pub fn process(input: ArgumentsProcessorInput)
        -> Result<ValuesPayload, ArgumentValueExtractorError> {
        return match input.payload {
            Value::Object(payload) =>
                ArgumentValuesExtractor::do_process(payload, &input.definitions),
            _ => Result::Err(ArgumentValueExtractorError::InvalidJsonInput)
        };
    }

    fn do_process(payload: &Map<String, Value>,
                  definitions: &HashMap<String, ArgumentDefinition>)
                  -> Result<ValuesPayload, ArgumentValueExtractorError> {
        let mut response: HashMap<String, ValueHolder> = HashMap::new();
        for (name, description) in definitions.iter() {
            let opt_argument = payload.get(name);
            match opt_argument {
                None => return Result::Err(
                    ArgumentValueExtractorError::MissingArgument(name.clone())),
                Some(value) => {
                    match ArgumentValuesExtractor::handle(description, value) {
                        Ok(holder) => {
                            response.insert(name.clone(), holder);
                        },
                        Err(policy) =>
                            return Result::Err(
                                ArgumentValueExtractorError::ExtractionFailure(
                                    name.clone(), value.clone(), policy)),
                    }
                },
            }
        }
        return Result::Ok(ValuesPayload::new(response));
    }

    fn handle(definition: &ArgumentDefinition,
              value: &Value)
              -> Result<ValueHolder, ValueExtractionPolicy> {
        ArgumentValueExtractor::extract(
            &ValueExtractorInput::new(
                value,
                definition.get_argument_type(),
                definition.get_extraction_policy()))
    }

}