use std::collections::HashMap;
use std::ops::Div;
use std::str::FromStr;

use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};



// Note: this implementation can have consistency issues as Eq and Hash are
// based on big rational implementation and actual use, e.g. geo location to
// time zone is based on a crude f64 approximation.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct GeoCoordinates {

    latitude: BigRational,
    longitude: BigRational

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

    pub(crate) fn new(latitude: BigRational, longitude: BigRational) -> Result<GeoCoordinates, ()> {
        if !GeoCoordinates::is_valid_latitude(&latitude)
            || !GeoCoordinates::is_valid_longitude(&longitude) {
            return Result::Err(());
        }
        Result::Ok(GeoCoordinates {
            latitude,
            longitude
        })
    }

    fn parse(opt: Option<&&str>) -> Result<BigRational, ()> {
        match opt {
            Some(val) => {
                match val.parse::<BigRational>() {
                    Ok(parsed) => Result::Ok(parsed),
                    Err(_) => Result::Err(())
                }
            },
            None => Result::Err(())
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


}

impl FromStr for GeoCoordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lat_long: Vec<&str> = s
            .split(",")
            .collect();
        return match GeoCoordinates::parse(lat_long.get(0)) {
            Ok(latitude) =>
                return match GeoCoordinates::parse(lat_long.get(1)) {
                    Ok(longitude) => GeoCoordinates::new(latitude, longitude),
                    Err(_) => Result::Err(()),
                },
            Err(_) => Result::Err(()),
        };
    }

}