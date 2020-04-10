use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use chrono::Weekday;
use chrono_tz::Tz;
use isolang::Language;
use serde::{Deserialize, Serialize};

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

    name: String,
    value: Tz

}

impl Wrapper<Tz> for TzWrapper {

    fn new(value: Tz) -> Self {
        TzWrapper {
            name: value.name().to_string(),
            value
        }
    }

    fn get(&self) -> &Tz {
        &self.value
    }

}

impl PartialOrd for TzWrapper {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }

}

impl Hash for TzWrapper {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }

}

impl PartialEq for TzWrapper {

    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

}

impl Eq for TzWrapper {}

#[derive(Debug, Clone)]
pub struct LanguageWrapper {

    code: String,
    value: Language

}

impl Wrapper<Language> for LanguageWrapper {

    fn new(value: Language) -> Self {
        LanguageWrapper {
            code: value.to_639_3().to_string(),
            value
        }
    }

    fn get(&self) -> &Language {
        &self.value
    }

}

impl Hash for LanguageWrapper {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }

}

impl PartialEq for LanguageWrapper {

    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

}

impl Eq for LanguageWrapper {}

impl PartialOrd for LanguageWrapper {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(&other.code)
    }

}