use std::collections::HashMap;

use serde_json::{Map, Value};
use strum_macros::AsRefStr;

use crate::arguments::description::ArgumentDescription;
use crate::arguments::extractors::{ValueExtractionPolicy, ValueExtractorInput};
use crate::arguments::values::ValueHolder;

pub mod extractors;
pub mod values;
pub mod description;

pub struct ArgumentsProcessorInput {

    descriptions: HashMap<String, ArgumentDescription>,
    payload: Value

}

pub enum ArgumentValueExtractorError {

    MissingArgument(String),
    ExtractionFailure(String, ValueExtractionPolicy),
    InvalidJsonInput

}

pub struct ArgumentValuesExtractor;

impl ArgumentValuesExtractor {

    pub fn process(input: ArgumentsProcessorInput)
        -> Result<HashMap<String, ValueHolder>, ArgumentValueExtractorError> {
        return match input.payload {
            Value::Object(payload) =>
                ArgumentValuesExtractor::do_process(&payload, &input.descriptions),
            _ => Result::Err(ArgumentValueExtractorError::InvalidJsonInput)
        };
    }

    fn do_process(payload: &Map<String, Value>,
                  descriptions: &HashMap<String, ArgumentDescription>)
        -> Result<HashMap<String, ValueHolder>, ArgumentValueExtractorError> {
        let mut response: HashMap<String, ValueHolder> = HashMap::new();
        for (name, description) in descriptions.iter() {
            println!("Here");
            let opt_argument = payload.get(name);
            match opt_argument {
                None => return Result::Err(
                    ArgumentValueExtractorError::MissingArgument(name.clone())),
                Some(value) => {
                    match ArgumentValuesExtractor::handle(description, value) {
                        Ok(holder) => response.insert(name.clone(), holder),
                        Err(policy) => return Result::Err(
                            ArgumentValueExtractorError::ExtractionFailure(name.clone(), policy)),
                    }
                },
            }
        }
        return Result::Ok(response);
    }

    fn handle(description: &ArgumentDescription, value: &Value)
        -> Result<ValueHolder, ValueExtractionPolicy> {
        Result::Err(ValueExtractionPolicy::Strict)
    }

}