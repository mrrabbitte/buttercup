use chrono::{Datelike, TimeZone};

use crate::transformations::mono::MonoInputTransformer;
use crate::transformations::transformer::{InputOrder, TransformationError};
use crate::values::{ValueHolder, ValueType};
use crate::values::zoned_date_time::ZonedDateTime;

pub struct DayOfWeekFromDateTimeRetrieval;

const DATELIKE_INPUT_TYPES: [ValueType; 3] =
    [ValueType::LocalDateTime, ValueType::LocalDate, ValueType::ZonedDateTime];

const DAY_OF_WEEK_RESULT_TYPE: ValueType = ValueType::DayOfWeek;

impl MonoInputTransformer for DayOfWeekFromDateTimeRetrieval {

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match value {
            ValueHolder::LocalDateTime(date_time) =>
                Result::Ok(ValueHolder::DayOfWeek(date_time.weekday())),
            ValueHolder::LocalDate(date) =>
                Result::Ok(ValueHolder::DayOfWeek(date.weekday())),
            ValueHolder::ZonedDateTime(zdt) =>
                Result::Ok(ValueHolder::DayOfWeek(zdt.get_date_time().weekday())),
            _ => Result::Err(
                TransformationError::InvalidInputType(value.clone(), InputOrder::First))
        }
    }

    fn get_input_types(&self) -> &'static [ValueType] {
        &DATELIKE_INPUT_TYPES
    }

    fn get_result_type(&self) -> &'static ValueType {
        &DAY_OF_WEEK_RESULT_TYPE
    }

}

impl DayOfWeekFromDateTimeRetrieval {

    const INSTANCE: DayOfWeekFromDateTimeRetrieval = DayOfWeekFromDateTimeRetrieval{};

    pub fn instance() -> &'static  DayOfWeekFromDateTimeRetrieval {
        &DayOfWeekFromDateTimeRetrieval::INSTANCE
    }

}
