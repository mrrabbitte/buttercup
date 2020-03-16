use crate::values::{ValueHolder, ValueType};

pub trait SingleValueTransformation {

    fn transform(value: ValueHolder) -> ValueHolder;
    fn get_value_type() -> ValueType;

}

pub trait DoubleValueTransformation {

    fn transform(first: ValueHolder,
                 second: ValueHolder) -> ValueHolder;
    fn get_first_value_type() -> ValueType;
    fn get_second_value_type() -> ValueType;

}