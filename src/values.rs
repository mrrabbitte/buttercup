use std::collections::HashMap;
use std::slice::Split;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use chrono_tz::Tz;
use num::bigint::BigInt;
use num::rational::BigRational;
use strum_macros::AsRefStr;

use crate::values::geolocation::GeoCoordinates;

pub mod transformations;
pub mod geolocation;

#[derive(AsRefStr, Debug)]
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
    GeoCoordinates(GeoCoordinates),

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
    DayOfWeek,
    TimeZone,
    GeoCoordinates

}

impl ValueType {

    fn matches(&self,
               value_holder: &ValueHolder) -> bool {
        self.as_ref() == value_holder.as_ref()
    }

}

#[derive(Debug)]
pub struct ValuesPayload {

    values: HashMap<String, ValueHolder>

}

impl ValuesPayload {

    pub fn new(values: HashMap<String, ValueHolder>) -> ValuesPayload {
        ValuesPayload {
            values
        }
    }

}
