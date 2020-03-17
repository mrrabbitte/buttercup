use chrono::Datelike;

use crate::values::{ValueHolder, ValueType};
use crate::values::transformations::{SingleValueTransformation, TransformationError};

pub struct DayOfWeekFromDateTimeRetrieval;

const DAY_OF_WEEK_INPUTS: [ValueType; 2] = [ValueType::LocalDateTime, ValueType::LocalDate];

impl SingleValueTransformation for DayOfWeekFromDateTimeRetrieval {

    fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match value {
            ValueHolder::LocalDateTime(date_time) =>
                Result::Ok(ValueHolder::DayOfWeek(date_time.weekday())),
            ValueHolder::LocalDate(date) =>
                Result::Ok(ValueHolder::DayOfWeek(date.weekday())),
            _ => Result::Err(TransformationError::InvalidInputType)
        }
    }

    fn get_input_value_type() -> &'static [ValueType] {
        &DAY_OF_WEEK_INPUTS
    }

    fn get_result_value_type() -> ValueType {
        ValueType::DayOfWeek
    }

}