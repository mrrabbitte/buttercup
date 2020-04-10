use std::cmp::Ordering;

use chrono::Weekday;
use chrono_tz::Tz;
use isolang::Language;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

pub trait Wrapper<T> {

    fn new(value: T) -> Self;
    fn get(&self) -> &T;

}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
pub struct WeekdayWrapper {

    value: Weekday,
    number_from_monday: u8

}

impl Wrapper<Weekday> for WeekdayWrapper {

    fn new(value: Weekday) -> Self {
        WeekdayWrapper {
            value,
            number_from_monday: value.number_from_monday() as u8
        }
    }

    fn get(&self) -> &Weekday {
        &self.value
    }

}

impl PartialOrd for WeekdayWrapper {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number_from_monday.partial_cmp(&other.number_from_monday)
    }

}

impl PartialEq for WeekdayWrapper {

    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TzWrapper {

    str_value: String,
    value: Tz

}

impl Wrapper<Tz> for TzWrapper {

    fn new(value: Tz) -> Self {
        TzWrapper {
            str_value: format!("{:?}", value),
            value
        }
    }

    fn get(&self) -> &Tz {
        &self.value
    }

}

impl PartialOrd for TzWrapper {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.str_value.partial_cmp(&other.str_value)
    }

}

impl Hash for TzWrapper {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }

}

impl PartialEq for TzWrapper {

    fn eq(&self, other: &Self) -> bool {
        self.value.eq(other.value)
    }

}

impl Eq for TzWrapper {

}

pub struct LanguageWrapper {

    str_value: String,
    value: Language

}

impl Wrapper<Language> for LanguageWrapper {

    fn new(value: Language) -> Self {
        LanguageWrapper {
            str_value: value.to_639_3(),
            value
        }
    }

    fn get(&self) -> &Language {
        &self.value
    }

}
