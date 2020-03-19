use crate::values::{ValueHolder, ValueType};
use crate::transformations::date_time::DayOfWeekFromDateTimeRetrieval;

pub mod date_time;
pub mod astro;
pub mod geolocation;

pub enum InputOrder {

    First,
    Second

}

pub enum TransformationError {

    InvalidInputType(ValueHolder, InputOrder)

}

pub enum SingleValueTransformation {

    DayOfWeekFromDateTimeRetrieval

}


impl SingleValueTransformation {

    pub fn is_input_value_type_ok(&self,
                                  value_type: &ValueType) -> bool {
        match self {
            SingleValueTransformation::DayOfWeekFromDateTimeRetrieval(transformer),
        }
    }

}

pub trait SingleValueTransformer {

    fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn is_input_value_type_ok(value_type: &ValueType) -> bool;
    fn get_result_type() -> ValueType;

}

pub trait DoubleValueTransformer {

    fn transform(first: ValueHolder,
                 second: ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn is_first_input_value_type_ok(value_type: &ValueType) -> bool;
    fn is_second_input_value_type_ok(value_type: &ValueType) -> bool;
    fn get_result_type() -> ValueType;

}