use std::collections::HashMap;
use std::slice::Split;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use chrono_tz::Tz;
use num::bigint::BigInt;
use num::rational::BigRational;
use strum_macros::{AsRefStr, EnumVariantNames};
use strum::VariantNames;

use crate::app::values::geolocation::GeoCoordinates;
use crate::app::values::zoned_date_time::ZonedDateTime;

pub mod geolocation;
pub mod zoned_date_time;
pub mod extractors;

#[derive(AsRefStr, EnumVariantNames, PartialEq, PartialOrd, Debug, Clone)]
pub enum ValueHolder {

    Boolean(bool),
    String(String),
    Decimal(BigRational),
    Integer(BigInt),
    LocalDateTime(NaiveDateTime),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
    DayOfWeek(Weekday),
    TimeZone(Tz),
    ZonedDateTime(ZonedDateTime),
    GeoCoordinates(GeoCoordinates),

}

#[derive(AsRefStr, EnumVariantNames, Debug, PartialEq)]
pub enum ValueType {

    Boolean,
    String,
    Decimal,
    Integer,
    LocalDateTime,
    LocalDate,
    LocalTime,
    DayOfWeek,
    TimeZone,
    ZonedDateTime,
    GeoCoordinates,

}

impl ValueType {

    fn matches(&self,
               value_holder: &ValueHolder) -> bool {
        self.as_ref() == value_holder.as_ref()
    }

}

#[derive(Debug, Clone)]
pub struct ValuesPayload {

    values: HashMap<String, ValueHolder>

}

impl ValuesPayload {

    pub fn new(values: HashMap<String, ValueHolder>) -> ValuesPayload {
        ValuesPayload {
            values
        }
    }

    pub fn get_values(&self) -> &HashMap<String, ValueHolder> {
        &self.values
    }

    pub fn get(&self,
               key: &String) -> Option<&ValueHolder> {
        self.values.get(key)
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_consistency() {
        assert_eq!(ValueHolder::VARIANTS, ValueType::VARIANTS);
    }

}