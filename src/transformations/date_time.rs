use chrono::{Datelike, TimeZone};

use crate::transformations::{DoubleInputTransformer, InputOrder, SingleInputTransformer, TransformationError};
use crate::values::{ValueHolder, ValueType};
use crate::values::zoned_date_time::ZonedDateTime;

pub struct DayOfWeekFromDateTimeRetrieval;

const DATELIKE_INPUT_TYPES: [ValueType; 3] =
    [ValueType::LocalDateTime, ValueType::LocalDate, ValueType::ZonedDateTime];

const DAY_OF_WEEK_RESULT_TYPE: ValueType = ValueType::DayOfWeek;

impl SingleInputTransformer for DayOfWeekFromDateTimeRetrieval {

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

pub struct LocalToZonedDateTime;

const DATE_TIME_INPUT_TYPE: [ValueType; 1] = [ValueType::LocalDateTime];
const TIMEZONE_INPUT_TYPE: [ValueType; 1] = [ValueType::TimeZone];

const ZDT_RESULT_TYPE: ValueType = ValueType::DayOfWeek;

impl DoubleInputTransformer for LocalToZonedDateTime {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match first {
            ValueHolder::LocalDateTime(date_time) => match second {
                ValueHolder::TimeZone(time_zone) =>
                    Result::Ok(
                        ValueHolder::ZonedDateTime(
                            ZonedDateTime::new(*date_time, *time_zone))
                    ),
                _ => Result::Err(
                    TransformationError::InvalidInputType(first.clone(), InputOrder::Second))
            },
            _ => Result::Err(
                TransformationError::InvalidInputType(first.clone(), InputOrder::First))
        };
    }

    fn get_first_input_types(&self) -> &'static [ValueType] {
        &DATE_TIME_INPUT_TYPE
    }

    fn get_second_input_types(&self) -> &'static [ValueType] {
        &TIMEZONE_INPUT_TYPE
    }

    fn get_result_type(&self) -> &'static ValueType {
        &ZDT_RESULT_TYPE
    }
}