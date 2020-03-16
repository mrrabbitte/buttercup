use strum_macros::AsRefStr;

use crate::arguments::values::ValueHolder;

pub mod extractors;
pub mod values;

pub struct ArgumentDescription {

    id: i32,
    name: String,
    argument_type: ValueType

}

#[derive(AsRefStr, Debug)]
pub enum ValueType {

    Boolean,
    String,
    Decimal,
    Integer,
    LocalDateTime,
    LocalDate,
    LocalTime,
    LatLong,
    DayOfWeek

}

impl ValueType {

    fn matches(&self,
               value_holder: &ValueHolder) -> bool {
        self.as_ref() == value_holder.as_ref()
    }

}

pub struct ArgumentProcessor {



}