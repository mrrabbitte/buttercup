use std::collections::HashMap;

use serde_json::{Map, Value};

use serde::{Serialize, Deserialize};

use crate::app::arguments::ArgumentDefinition;
use crate::app::values::{ValueHolder, ValuesPayload};
use crate::app::values::extractors::{ValueExtractionError, ValueExtractorInput, ValueExtractorService};

pub struct ArgumentsExtractionInput<'a> {

    definitions: &'a HashMap<String, ArgumentDefinition>,
    payload: &'a Value

}

impl<'a> ArgumentsExtractionInput<'a> {

    pub fn new(definitions: &'a HashMap<String, ArgumentDefinition>,
               payload: &'a Value) -> ArgumentsExtractionInput<'a> {
        ArgumentsExtractionInput {
            definitions,
            payload
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentValueExtractorError {

    MissingArgument(String),
    ExtractionFailure(String, Value, ValueExtractionError),
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
                        Err(error) =>
                            return Result::Err(
                                ArgumentValueExtractorError::ExtractionFailure(
                                    name.clone(), value.clone(), error)),
                    }
                },
            }
        }
        return Result::Ok(ValuesPayload::new(response));
    }

    fn handle(definition: &ArgumentDefinition,
              value: &Value)
              -> Result<ValueHolder, ValueExtractionError> {
        ValueExtractorService::extract(
            &ValueExtractorInput::new(
                value,
                definition.get_argument_type(),
                definition.get_extraction_policy()))
    }

}