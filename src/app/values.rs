use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::ops::Deref;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use isocountry::CountryCode;
use num::bigint::BigInt;
use num::rational::BigRational;
use serde::{Deserialize, Serialize};
use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{AsRefStr, EnumIter, EnumVariantNames};

use crate::app::values::email::Email;
use crate::app::values::geolocation::GeoCoordinates;
use crate::app::values::lists::ValueHoldersList;
use crate::app::values::wrappers::{LanguageWrapper, TzWrapper, WeekdayWrapper};
use crate::app::values::zoned_date_time::ZonedDateTime;
use std::time::Duration;
use std::convert::TryFrom;
use crate::app::variables::VariableName;

pub mod email;
pub mod extractors;
pub mod geolocation;
pub mod lists;
pub mod wrappers;
pub mod zoned_date_time;

#[derive(Serialize, Deserialize, AsRefStr, EnumVariantNames, Eq, Hash, PartialEq, PartialOrd,
    Debug, Clone)]
pub enum ValueHolder {

    Boolean(bool),
    Country(CountryCode),
    DayOfWeek(WeekdayWrapper),
    Decimal(BigRational),
    Duration(Duration),
    Email(Email),
    GeoCoordinates(GeoCoordinates),
    Integer(BigInt),
    IpAddress(IpAddr),
    Language(LanguageWrapper),
    List(ValueHoldersList),
    LocalDate(NaiveDate),
    LocalDateTime(NaiveDateTime),
    LocalTime(NaiveTime),
    TimeZone(TzWrapper),
    String(String),
    ZonedDateTime(ZonedDateTime),

}

impl TryFrom<ValueHolder> for Duration {
    type Error = ();

    fn try_from(value: ValueHolder) -> Result<Self, Self::Error> {
        match value {
            ValueHolder::Duration(duration) => Result::Ok(duration),
            _ => Result::Err(())
        }
    }
}

impl ValueHolder {

    pub fn contains(&self,
                    other: &ValueHolder) -> bool {
        match (self, other) {
            (ValueHolder::String(this), ValueHolder::String(other)) =>
                this.contains(other),
            (ValueHolder::List(list), _) =>
                list.get_value_type().matches(other)
                    && list.get_elements().contains(other),
            (_, _) => false
        }
    }

    pub fn ends_with(&self,
                     other: &ValueHolder) -> bool {
        match (self, other) {
            (ValueHolder::String(this), ValueHolder::String(other)) =>
                this.ends_with(other),
            (_, _) => false
        }
    }

    pub fn is_in(&self,
                 other:&ValueHolder) -> bool {
        other.contains(self)
    }

    pub fn starts_with(&self,
                       other: &ValueHolder) -> bool {
        match (self, other) {
            (ValueHolder::String(this), ValueHolder::String(other)) =>
                this.starts_with(other),
            (_, _) => false
        }
    }

}

#[derive(AsRefStr, EnumVariantNames, EnumIter,
    Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ValueType {

    Boolean,
    Country,
    DayOfWeek,
    Decimal,
    Duration,
    Email,
    GeoCoordinates,
    Integer,
    IpAddress,
    Language,
    List,
    LocalDate,
    LocalDateTime,
    LocalTime,
    TimeZone,
    String,
    ZonedDateTime,

}

lazy_static! {
        static ref ALL_VALUE_TYPES: Vec<ValueType> = ValueType::iter().collect();
}

impl ValueType {

    pub fn all_value_types() -> &'static Vec<ValueType> {
        &ALL_VALUE_TYPES
    }

    fn matches(&self,
               value_holder: &ValueHolder) -> bool {
        self.as_ref() == value_holder.as_ref()
    }

}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ValuesPayload {

    values: HashMap<String, ValueHolder>,
    keys: HashSet<String>

}

impl ValuesPayload {

    pub fn new(values: HashMap<String, ValueHolder>) -> ValuesPayload {
        let keys = values.keys().cloned().collect();
        ValuesPayload {
            values,
            keys
        }
    }

    pub fn empty() -> ValuesPayload {
        ValuesPayload {
            values: HashMap::new(),
            keys: HashSet::new()
        }
    }

    pub fn get_values(&self) -> &HashMap<String, ValueHolder> {
        &self.values
    }

    pub fn get(&self,
               key: &String) -> Option<&ValueHolder> {
        self.values.get(key)
    }

    pub fn get_keys(&self) -> &HashSet<String> {
        &self.keys
    }

}



#[cfg(test)]
mod tests {
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
