use crate::values::{ValueHolder, ValueType};
use crate::values::transformations::{DoubleValueTransformation, TransformationError};

pub struct IsAfterSunset;

const IS_AFTER_SUNSET_FIRST_INPUTS: [ValueType; 1] = [ValueType::LocalDateTime];

const IS_AFTER_SUNSET_SECOND_INPUTS: [ValueType; 1] = [ValueType::LatLong];

impl DoubleValueTransformation for IsAfterSunset {

    fn transform(first: ValueHolder, second: ValueHolder)
        -> Result<ValueHolder, TransformationError> {
        unimplemented!()
    }

    fn get_first_input_value_type() -> &'static [ValueType] {
        &IS_AFTER_SUNSET_FIRST_INPUTS
    }

    fn get_second_input_value_type() -> &'static [ValueType] {
        &IS_AFTER_SUNSET_SECOND_INPUTS
    }

    fn get_result_value_type() -> ValueType {
        ValueType::Boolean
    }

}