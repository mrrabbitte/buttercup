use crate::transformations::{InputOrder, TransformationError};
use crate::transformations::bi::BiInputTransformer;
use crate::values::{ValueHolder, ValueType};
use crate::values::zoned_date_time::ZonedDateTime;

pub struct LocalToZonedDateTime;

const DATE_TIME_INPUT_TYPE: [ValueType; 1] = [ValueType::LocalDateTime];
const TIMEZONE_INPUT_TYPE: [ValueType; 1] = [ValueType::TimeZone];

const ZDT_RESULT_TYPE: ValueType = ValueType::DayOfWeek;

impl BiInputTransformer for LocalToZonedDateTime {

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