use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::values::{GeoCoordinates, ValueHolder, ValueType};
use crate::values::transformations::{DoubleValueTransformation, TransformationError};

pub struct IsNight;

mod sun_position;

const SUN_POSITION_FIRST_INPUT: [ValueType; 1] = [ValueType::LocalDateTime];

const SUN_POSITION_SECOND_INPUT: [ValueType; 1] = [ValueType::TimeZone];

const SUN_POSITION_THIRD_INPUT: [ValueType; 1] = [ValueType::GeoCoordinates];

impl DoubleValueTransformation for IsNight {

    fn transform(first: ValueHolder, second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        unimplemented!()
    }

    fn get_first_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_FIRST_INPUT
    }

    fn get_second_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_SECOND_INPUT
    }

    fn get_result_value_type() -> ValueType {
        ValueType::Boolean
    }

}

