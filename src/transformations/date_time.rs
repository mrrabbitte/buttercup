use chrono::Datelike;

use crate::transformations::{InputOrder, SingleValueTransformer, TransformationError};
use crate::values::{ValueHolder, ValueType};

pub struct DayOfWeekFromDateTimeRetrieval;

impl SingleValueTransformer for DayOfWeekFromDateTimeRetrieval {

    pub fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError> {
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

    pub fn is_input_value_type_ok(value_type: &ValueType) -> bool {
        ValueType::LocalDateTime == *value_type
            || ValueType::LocalDate == *value_type
            || ValueType::ZonedDateTime == *value_type
    }

    pub fn get_result_type() -> ValueType {
        ValueType::DayOfWeek
    }

}

