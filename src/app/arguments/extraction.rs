use std::collections::HashMap;

use serde_json::{Map, Value};
use strum_macros::AsRefStr;

use crate::app::values::extractors::{ValueExtractorService, ValueExtractionPolicy, ValueExtractorInput};
use crate::app::values::{ValueHolder, ValuesPayload};
use crate::app::arguments::ArgumentDefinition;

pub struct ArgumentsExtractionInput<'a> {

    definitions: &'a HashMap<String, ArgumentDefinition>,
    payload: &'a Value

}

impl ArgumentsExtractionInput<'_> {

    pub fn new(definitions: &HashMap<String, ArgumentDefinition>,
               payload: &Value) -> ArgumentsExtractionInput {
        ArgumentsExtractionInput {
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

pub struct ArgumentValuesExtractionService;

impl ArgumentValuesExtractionService {

    pub fn process(input: ArgumentsExtractionInput)
                   -> Result<ValuesPayload, ArgumentValueExtractorError> {
        return match input.payload {
            Value::Object(payload) =>
                ArgumentValuesExtractionService::do_process(payload, &input.definitions),
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
                    match ArgumentValuesExtractionService::handle(description, value) {
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
        ValueExtractorService::extract(
            &ValueExtractorInput::new(
                value,
                definition.get_argument_type(),
                definition.get_extraction_policy()))
    }

}