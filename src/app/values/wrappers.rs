use std::cmp::Ordering;

use chrono::Weekday;
use chrono_tz::Tz;

pub trait Wrapper<T> {

    fn new(value: T) -> Self;
    fn get(&self) -> &T;

}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct TzWrapper {

    value: Tz

}

impl Wrapper<Tz> for TzWrapper {

    fn new(value: Tz) -> Self {
        TzWrapper {
            value
        }
    }

    fn get(&self) -> &Tz {
        &self.value
    }

}

impl PartialOrd for TzWrapper {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let first = format!("{:?}", self);
        let other = format!("{:?}", other);
        first.partial_cmp(&other)
    }

}
