use strum_macros::AsRefStr;

use crate::arguments::values::ValueHolder;

pub mod extractors;
pub mod transformers;
pub mod values;

pub struct ArgumentDescription {

    id: i32,
    name: String,
    argument_type: ArgumentType

}

#[derive(AsRefStr)]
pub enum ArgumentType {

    Boolean,
    String,
    Decimal,
    Integer,
    LocalDateTime,
    LocalDate,
    LocalTime,
    LatLong

}

impl ArgumentType {

    fn matches(&self,
               value_holder: &ValueHolder) -> bool {
        self.as_ref() == value_holder.as_ref()
    }

}

