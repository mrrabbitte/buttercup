use chrono::{Datelike, TimeZone, Weekday};

use crate::app::transformations::mono::MonoInputTransformer;
use crate::app::transformations::transformer::{InputOrder, TransformationError};
use crate::app::values::{ValueHolder, ValueType};
use crate::app::values::wrappers::{WeekdayWrapper, Wrapper};
use crate::app::values::zoned_date_time::ZonedDateTime;

pub struct DayOfWeekFromDateTimeRetrieval;

const DATELIKE_INPUT_TYPES: [ValueType; 3] =
    [ValueType::LocalDateTime, ValueType::LocalDate, ValueType::ZonedDateTime];

const DAY_OF_WEEK_RESULT_TYPE: ValueType = ValueType::DayOfWeek;

impl DayOfWeekFromDateTimeRetrieval {

    fn ok(weekday: Weekday) -> Result<ValueHolder, TransformationError> {
        Result::Ok(ValueHolder::DayOfWeek(WeekdayWrapper::new(weekday)))
    }

}

impl MonoInputTransformer for DayOfWeekFromDateTimeRetrieval {

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match value {
            ValueHolder::LocalDateTime(date_time) =>
                DayOfWeekFromDateTimeRetrieval::ok(date_time.weekday()),
            ValueHolder::LocalDate(date) =>
                DayOfWeekFromDateTimeRetrieval::ok(date.weekday()),
            ValueHolder::ZonedDateTime(zdt) =>
                DayOfWeekFromDateTimeRetrieval::ok(zdt.get_date_time().weekday()),
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
