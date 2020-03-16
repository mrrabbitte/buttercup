use std::slice::Split;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use num::bigint::BigInt;
use num::rational::BigRational;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
pub enum ValueHolder {

    Boolean(bool),
    String(String),
    Decimal(BigRational),
    Integer(BigInt),
    LocalDateTime(NaiveDateTime),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
    LatLong(GeoCoordinates),
    DayOfWeek(Weekday)

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

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoCoordinates {

    latitude: f64,
    longitude: f64

}

impl GeoCoordinates {

    pub fn new(latitude: f64, longitude: f64) -> GeoCoordinates {
        GeoCoordinates {
            latitude,
            longitude
        }
    }

    fn parse(opt: Option<&&str>) -> Option<f64> {
        match opt {
            Some(val) => {
                match val.parse::<f64>() {
                    Ok(float_value) => Option::Some(float_value),
                    Err(_) => Option::None
                }
            },
            None => Option::None
        }
    }

}

impl FromStr for GeoCoordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lat_long: Vec<&str> = s
            .split(",")
            .collect();
        let lat_opt = GeoCoordinates::parse(lat_long.get(0));
        if lat_opt.is_none() {
            return Result::Err(());
        }
        let lon_opt = GeoCoordinates::parse(lat_long.get(1));
        if lon_opt.is_none() {
            return Result::Err(());
        }
        Result::Ok(
            GeoCoordinates::new(
                lat_opt.unwrap(), lon_opt.unwrap()))
    }

}