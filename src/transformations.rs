use crate::transformations::bi::BiInputTransformation;
use crate::transformations::mono::MonoInputTransformation;
use crate::values::geolocation::GeoCoordinates;
use crate::values::ValueHolder;

pub mod mono;
pub mod bi;

pub enum InputOrder {

    First,
    Second

}

pub enum TransformationError {

    InvalidInputType(ValueHolder, InputOrder),
    CouldNotFindTimezone(GeoCoordinates),
    UnknownTimezone(String)

}

pub enum TransformationType {

    SingleInput,
    DoubleInput

}

pub struct TransformationDefinition {

    id: i32,
    transformation_type: TransformationType,
    result_value_name: String

}

pub struct SingleInputTransformationDefinition {

    transformation_definition_id: i32,
    input_name: String,
    transformation: MonoInputTransformation

}

pub struct DoubleInputTransformationDefinition {

    transformation_definition_id: i32,
    first_input_name: String,
    second_input_name: String,
    transformation: MonoInputTransformation

}

pub enum Transformation {

    SingleInputTransformation(MonoInputTransformation),
    DoubleInputTransformation(BiInputTransformation)

}

pub struct TransformationRequest {

    definition: TransformationDefinition,
    transformation: Transformation

}

