#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::net::IpAddr;
use std::time::Duration;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use isocountry::CountryCode;
use num::bigint::BigInt;
use num::rational::BigRational;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumVariantNames};

use crate::email::Email;
use crate::geolocation::GeoCoordinates;
use crate::lists::ValueHoldersList;
use crate::wrappers::{LanguageWrapper, TzWrapper, WeekdayWrapper};
use crate::zoned_date_time::ZonedDateTime;
use std::sync::Arc;

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
    List(Arc<ValueHoldersList>),
    LocalDate(NaiveDate),
    LocalDateTime(NaiveDateTime),
    LocalTime(NaiveTime),
    TimeZone(TzWrapper),
    String(Arc<String>),
    ZonedDateTime(ZonedDateTime),

}

impl ValueHolder {

    pub fn contains(&self,
                    other: &ValueHolder) -> bool {
        match (self, other) {
            (ValueHolder::String(this), ValueHolder::String(other)) =>
                this.as_ref().contains(other.as_ref()),
            (ValueHolder::List(list), _) => {
                let list_ref = list.as_ref();
                list_ref.get_value_type().matches(other)
                    && list_ref.get_elements().contains(other)
            },
            (_, _) => false
        }
    }

    pub fn ends_with(&self,
                     other: &ValueHolder) -> bool {
        match (self, other) {
            (ValueHolder::String(this), ValueHolder::String(other)) =>
                this.as_ref().ends_with(other.as_ref()),
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
                this.as_ref().starts_with(other.as_ref()),
            (_, _) => false
        }
    }

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

impl From<String> for ValueHolder {
    fn from(val: String) -> Self {
        ValueHolder::String(Arc::new(val))
    }
}

impl From<&str> for ValueHolder {
    fn from(val: &str) -> Self {
        ValueHolder::String(Arc::new(val.to_owned()))
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

    pub fn singleton(name: String, value: ValueHolder) -> ValuesPayload {
        let mut values = HashMap::new();
        values.insert(name, value);
        ValuesPayload::new(values)
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

    pub fn into_keys(self) -> HashSet<String> {
        self.keys
    }

}



#[cfg(test)]
mod tests {
    use num::FromPrimitive;
    use strum::VariantNames;

    use super::*;
    use num_rational::Ratio;

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
