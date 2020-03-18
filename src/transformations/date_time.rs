use chrono::Datelike;

use crate::transformations::{InputOrder, SingleValueTransformation, TransformationError};
use crate::values::{ValueHolder, ValueType};

pub struct DayOfWeekFromDateTimeRetrieval;

const DAY_OF_WEEK_INPUTS: [ValueType; 2] = [ValueType::LocalDateTime, ValueType::LocalDate];

impl SingleValueTransformation for DayOfWeekFromDateTimeRetrieval {

    fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match value {
            ValueHolder::LocalDateTime(date_time) =>
                Result::Ok(ValueHolder::DayOfWeek(date_time.weekday())),
            ValueHolder::LocalDate(date) =>
                Result::Ok(ValueHolder::DayOfWeek(date.weekday())),
            ValueHolder::ZonedDateTime(zdt) =>
                Result::Ok(ValueHolder::DayOfWeek(zdt.get_date_time().weekday())),
            _ => Result::Err(TransformationError::InvalidInputType(value, InputOrder::First))
        }
    }

    fn get_input_value_type() -> &'static [ValueType] {
        &DAY_OF_WEEK_INPUTS
    }

    fn get_result_value_type() -> ValueType {
        ValueType::DayOfWeek
    }

}
