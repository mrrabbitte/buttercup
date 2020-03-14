use std::slice::Split;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use strum_macros::AsRefStr;

use crate::arguments::ArgumentType;

#[derive(AsRefStr)]
pub enum ValueHolder<'a> {

    Boolean(bool),
    String(&'a str),
    Decimal(f64),
    Integer(i64),
    LocalDateTime(NaiveDateTime),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
    LatLong(GeoCoordinate)

}

pub struct GeoCoordinate {

    latitude: f64,
    longitude: f64

}

impl GeoCoordinate {

    pub fn new(latitude: f64, longitude: f64) -> GeoCoordinate {
        GeoCoordinate {
            latitude,
            longitude
        }
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

impl FromStr for GeoCoordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lat_long: Vec<&str> = s
            .split(",")
            .collect();
        let lat_opt = parse(lat_long.get(0));
        if lat_opt.is_none() {
            return Result::Err(());
        }
        let lon_opt = parse(lat_long.get(1));
        if lon_opt.is_none() {
            return Result::Err(());
        }
        Result::Ok(
            GeoCoordinate::new(
                lat_opt.unwrap(), lon_opt.unwrap()))
    }

}