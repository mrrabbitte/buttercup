use crate::values::{ValueHolder, ValueType};

pub mod date_time;
pub mod time_of_day;

pub enum TransformationError {

    InvalidInputType

}

pub trait SingleValueTransformation {

    fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_input_value_type() -> &'static [ValueType];
    fn get_result_value_type() -> ValueType;

}

pub trait DoubleValueTransformation {

    fn transform(first: ValueHolder,
                 second: ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_first_input_value_type() -> &'static [ValueType];
    fn get_second_input_value_type() -> &'static [ValueType];
    fn get_result_value_type() -> ValueType;

}