use std::collections::HashMap;
use std::str::FromStr;

use num::{BigInt, BigRational, FromPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::Div;

#[derive(Serialize, Deserialize, Eq, Hash, Debug, Clone, PartialEq, PartialOrd)]
pub struct GeoCoordinates {

    latitude: BigRational,
    longitude: BigRational

}

impl GeoCoordinates {

    pub fn get_latitude_as_f64(&self) -> &BigRational {
        &self.latitude.numer().div(&self.latitude.denom()).as_f64()
    }

    pub fn get_longitude_as_f64(&self) -> &BigRational {
        &self.longitude
    }

    pub fn is_valid(&self) -> bool {
        GeoCoordinates::is_valid_longitude(self.get_longitude())
            && GeoCoordinates::is_valid_latitude(self.get_latitude())
    }

    fn new(latitude: BigRational, longitude: BigRational) -> Result<GeoCoordinates, ()> {
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
                    Ok(float_value) => Result::Ok(float_value),
                    Err(_) => Result::Err(())
                }
            },
            None => Result::Err(())
        }
    }

    fn is_valid_latitude(latitude: &BigRational) -> bool {
        *latitude >= BigRational::from_integer(BigInt::from(-90))
            && *latitude <= BigRational::from_integer(BigInt::from(90))
    }

    fn is_valid_longitude(longitude: &BigRational) -> bool {
        *longitude >= BigRational::from_integer(BigInt::from(-180))
            && *longitude <= BigRational::from_integer(BigInt::from(180))
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