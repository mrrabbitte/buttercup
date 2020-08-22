use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::IpAddr;
use std::slice::Split;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use isocountry::CountryCode;
use num::bigint::BigInt;
use num::rational::BigRational;
use serde::{Deserialize, Serialize};
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumVariantNames};

use crate::app::values::email::Email;
use crate::app::values::geolocation::GeoCoordinates;
use crate::app::values::wrappers::{LanguageWrapper, TzWrapper, WeekdayWrapper};
use crate::app::values::zoned_date_time::ZonedDateTime;

pub mod geolocation;
pub mod zoned_date_time;
pub mod extractors;
pub mod wrappers;
pub mod email;

#[derive(Serialize, Deserialize, AsRefStr, EnumVariantNames, Eq, Hash, PartialEq, PartialOrd,
    Debug, Clone)]
pub enum ValueHolder {

    Boolean(bool),
    String(String),
    Decimal(BigRational),
    Integer(BigInt),
    LocalDateTime(NaiveDateTime),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
    DayOfWeek(WeekdayWrapper),
    TimeZone(TzWrapper),
    ZonedDateTime(ZonedDateTime),
    GeoCoordinates(GeoCoordinates),
    Language(LanguageWrapper),
    Country(CountryCode),
    Email(Email),
    IpAddress(IpAddr)

}

#[derive(AsRefStr, EnumVariantNames, Debug, PartialEq, Serialize, Deserialize)]
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
    Language,
    Country,
    Email,
    IpAddress

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

    use num::FromPrimitive;

    use super::*;

    #[test]
    fn test_consistency() {
        assert_eq!(ValueHolder::VARIANTS, ValueType::VARIANTS);
    }

    #[test]
    fn test_ne() {
        assert_ne!(ValueHolder::Decimal(BigRational::from_f64(0.321421).unwrap()),
                   ValueHolder::Decimal(BigRational::from_f64(0.321422).unwrap()));
        assert_ne!(ValueHolder::Decimal(BigRational::from_f64(0.0).unwrap()),
                   ValueHolder::Integer(BigInt::from(0)));

    }

}
