use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Div;
use std::str::FromStr;

use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// Note: this implementation can have consistency issues as Eq and Hash are
// based on big rational implementation and actual use, e.g. geo location to
// time zone is based on a crude f64 approximation.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct GeoCoordinates {

    latitude: BigRational,
    longitude: BigRational

}

#[derive(Serialize, Deserialize, Debug)]
pub enum GeoCoordinatesValueError {

    InvalidJsonStructure,
    InvalidCommaSeparatedStructure,
    InvalidLongitude,
    InvalidLatitude,
    MissingLongitude,
    MissingLatitude

}

impl GeoCoordinates {

    pub fn get_latitude_as_f64(&self) -> f64 {
        GeoCoordinates::rational_to_f64(&self.latitude).unwrap()
    }

    pub fn get_longitude_as_f64(&self) -> f64 {
        GeoCoordinates::rational_to_f64(&self.longitude).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        GeoCoordinates::is_valid_longitude(&self.latitude)
            && GeoCoordinates::is_valid_latitude(&self.longitude)
    }

    pub(crate) fn new(latitude: BigRational,
                      longitude: BigRational) -> Result<GeoCoordinates, GeoCoordinatesValueError> {
        if !GeoCoordinates::is_valid_latitude(&latitude) {
            return Result::Err(GeoCoordinatesValueError::InvalidLatitude);
        }
        if !GeoCoordinates::is_valid_longitude(&longitude) {
            return Result::Err(GeoCoordinatesValueError::InvalidLongitude);
        }
        Result::Ok(GeoCoordinates {
            latitude,
            longitude
        })
    }

    fn parse(val: &&str) -> Option<f64> {
        match val.parse::<f64>() {
            Ok(parsed) => Option::Some(parsed),
            Err(_) => Option::None
        }
    }

    fn is_valid_latitude(latitude: &BigRational) -> bool {
        GeoCoordinates::rational_to_f64(&latitude).is_ok()
            && *latitude >= BigRational::from_integer(BigInt::from(-90))
            && *latitude <= BigRational::from_integer(BigInt::from(90))
    }

    fn is_valid_longitude(longitude: &BigRational) -> bool {
        GeoCoordinates::rational_to_f64(&longitude).is_ok()
            && *longitude >= BigRational::from_integer(BigInt::from(-180))
            && *longitude <=  BigRational::from_integer(BigInt::from(180))
    }

    fn rational_to_f64(rational: &BigRational) -> Result<f64, ()> {
        return match rational.numer().to_i64() {
            None => Result::Err(()),
            Some(numerator) => match rational.denom().to_i64() {
                None => Result::Err(()),
                Some(denominator) =>
                    Result::Ok((numerator as f64) / (denominator as f64)),
            },
        }
    }

    fn from_values(latitude: &Value,
                   longitude: &Value) -> Result<GeoCoordinates, GeoCoordinatesValueError> {
        match GeoCoordinates::rational_from_value(latitude) {
            None => Result::Err(GeoCoordinatesValueError::InvalidLatitude),
            Some(lat) =>
                match GeoCoordinates::rational_from_value(longitude) {
                    None => Result::Err(GeoCoordinatesValueError::InvalidLongitude),
                    Some(long) => GeoCoordinates::new(lat, long),
                },
        }
    }

    fn rational_from_value(value: &Value) -> Option<BigRational> {
        match value {
            Value::Number(val) =>
                val
                    .as_f64()
                    .and_then(BigRational::from_f64),
            Value::String(val) =>
                GeoCoordinates::parse(&val.as_str())
                    .and_then(BigRational::from_f64),
            _ => Option::None
        }
    }

}

impl FromStr for GeoCoordinates {
    type Err = GeoCoordinatesValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lat_long: Vec<&str> = s
            .split(",")
            .collect();
        if lat_long.len() != 2 {
            return Result::Err(GeoCoordinatesValueError::InvalidCommaSeparatedStructure);
        }
        return match lat_long.get(0)
            .and_then(GeoCoordinates::parse)
            .and_then(BigRational::from_f64) {
            Some(latitude) =>
                return match lat_long.get(1)
                    .and_then(GeoCoordinates::parse)
                    .and_then(BigRational::from_f64) {
                    Some(longitude) =>
                        GeoCoordinates::new(latitude, longitude),
                    None => Result::Err(GeoCoordinatesValueError::InvalidLongitude),
                },
            None => Result::Err(GeoCoordinatesValueError::InvalidLatitude),
        };
    }

}

const ALLOWED_LATITUDE_NAMES: [&str; 3] = ["lat", "latitude", "la"];
const ALLOWED_LONGITUDE_NAMES: [&str; 4] = ["long", "longitude", "lon", "lo"];

struct LatLongGetter;

impl LatLongGetter {

    fn get_latitude(target: &Map<String, Value>) -> Option<&Value> {
        for name in &ALLOWED_LATITUDE_NAMES {
            let opt_for_name = target.get(&name.to_string());
            if opt_for_name.is_some() {
                return opt_for_name;
            }
        }
        Option::None
    }

    fn get_longitude(target: &Map<String, Value>) -> Option<&Value> {
        for name in &ALLOWED_LONGITUDE_NAMES {
            let opt_for_name = target.get(&name.to_string());
            if opt_for_name.is_some() {
                return opt_for_name;
            }
        }
        Option::None
    }

}

impl TryFrom<&Map<String, Value>> for GeoCoordinates {
    type Error = GeoCoordinatesValueError;

    fn try_from(value: &Map<String, Value>) -> Result<Self, Self::Error> {
        match LatLongGetter::get_latitude(&value) {
            None => Result::Err(GeoCoordinatesValueError::MissingLatitude),
            Some(latitude) => match LatLongGetter::get_longitude(&value) {
                None => Result::Err(GeoCoordinatesValueError::MissingLongitude),
                Some(longitude) => GeoCoordinates::from_values(latitude, longitude),
            },
        }
    }

}