use std::collections::HashMap;

use buttercup_values::{ValueHolder, ValuesPayload};
use buttercup_values::geolocation::GeoCoordinates;
use serde::{Deserialize, Serialize};

use crate::di::DiInputTransformation;
use crate::mono::MonoInputTransformation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputOrder {

    First,
    Second

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationError {

    InvalidInputType(ValueHolder, InputOrder),
    CouldNotFindValue(String),
    CouldNotFindTimezone(GeoCoordinates),
    UnknownTimezone(String)

}

#[derive(Serialize, Deserialize)]
pub enum TransformationType {

    SingleInput,
    DoubleInput

}

#[derive(Serialize, Deserialize)]
pub struct TransformationDefinition {

    id: i32,
    transformation_type: TransformationType,
    result_value_name: String

}

impl TransformationDefinition {

    pub fn new(id: i32,
               transformation_type: TransformationType,
               result_value_name: String) -> TransformationDefinition {
        TransformationDefinition {
            id,
            transformation_type,
            result_value_name
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct SingleInputTransformationDefinition {

    transformation_definition_id: i32,
    input_name: String,
    transformation: MonoInputTransformation

}

impl SingleInputTransformationDefinition {

    pub fn new(transformation_definition_id: i32,
               input_name: String,
               transformation: MonoInputTransformation) -> SingleInputTransformationDefinition {
        SingleInputTransformationDefinition {
            transformation_definition_id,
            input_name,
            transformation
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct DoubleInputTransformationDefinition {

    transformation_definition_id: i32,
    first_input_name: String,
    second_input_name: String,
    transformation: DiInputTransformation

}

impl DoubleInputTransformationDefinition {

    pub fn new(transformation_definition_id: i32,
               first_input_name: String,
               second_input_name: String,
               transformation: DiInputTransformation) -> DoubleInputTransformationDefinition {
        DoubleInputTransformationDefinition {
            transformation_definition_id,
            first_input_name,
            second_input_name,
            transformation
        }
    }

}

#[derive(Serialize, Deserialize)]
pub enum Transformation {

    Mono(SingleInputTransformationDefinition),
    Bi(DoubleInputTransformationDefinition)

}

#[derive(Serialize, Deserialize)]
pub struct TransformationRequest {

    definition: TransformationDefinition,
    transformation: Transformation

}

impl TransformationRequest {

    pub fn new(definition: TransformationDefinition,
               transformation: Transformation) -> TransformationRequest {
        TransformationRequest {
            definition,
            transformation
        }
    }

    pub fn new_mono(definition: TransformationDefinition,
                    transformation: SingleInputTransformationDefinition)
                    -> TransformationRequest {
        TransformationRequest::new(definition, Transformation::Mono(transformation))
    }

    pub fn new_di(definition: TransformationDefinition,
                  transformation: DoubleInputTransformationDefinition)
                  -> TransformationRequest {
        TransformationRequest::new(definition, Transformation::Bi(transformation))
    }

}

pub struct TransformationService;

impl TransformationService {

    pub fn transform(payload: &ValuesPayload,
                     transformation_requests: &Vec<TransformationRequest>)
                     -> Result<ValuesPayload, TransformationError> {
        let values = payload.get_values();
        let mut new_values: HashMap<String, ValueHolder> = values.clone();
        for request in transformation_requests {
            let result = match &request.transformation {
                Transformation::Mono(def)
                => TransformationService::handle_single(def, &new_values),
                Transformation::Bi(
                    def)
                => TransformationService::handle_double(def, &new_values),
            };
            match result {
                Ok(new_value) =>
                    {
                        new_values.insert(
                            request.definition.result_value_name.clone(),
                            new_value);
                    },
                Err(err) => {
                    return Result::Err(err);
                },
            };
        }
        return Result::Ok(ValuesPayload::new(new_values));
    }

    pub fn initialize() {
        MonoInputTransformation::initialize();
        DiInputTransformation::initialize();
    }

    fn handle_single(definition: &SingleInputTransformationDefinition,
                     values: &HashMap<String, ValueHolder>)
                     -> Result<ValueHolder, TransformationError> {
        let value_name = &definition.input_name;
        return match values.get(value_name) {
            Some(value) => definition.transformation.transform(value),
            None => Result::Err(
                TransformationError::CouldNotFindValue(value_name.clone())),
        };
    }

    fn handle_double(definition: &DoubleInputTransformationDefinition,
                     values: &HashMap<String, ValueHolder>)
                     -> Result<ValueHolder, TransformationError> {
        let first_value_name = &definition.first_input_name;
        return match values.get(first_value_name) {
            Some(first_value) => {
                let second_value_name = &definition.second_input_name;
                return match values.get(second_value_name) {
                    Some(second_value) =>
                        definition.transformation.transform(first_value, second_value),
                    None => Result::Err(
                        TransformationError::CouldNotFindValue(second_value_name.clone()))
                };
            }
            None => Result::Err(
                TransformationError::CouldNotFindValue(first_value_name.clone())),
        };
    }
}