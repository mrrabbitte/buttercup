use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoCoordinates {

    latitude: f64,
    longitude: f64

}

impl GeoCoordinates {

    pub fn get_latitude(&self) -> &f64 {
        &self.latitude
    }

    pub fn get_longitude(&self) -> &f64 {
        &self.longitude
    }

    pub fn is_valid(&self) -> bool {
        GeoCoordinates::is_valid_latitude(self.latitude)
            && GeoCoordinates::is_valid_longitude(self.longitude)
    }

    fn new(latitude: f64, longitude: f64) -> Result<GeoCoordinates, ()> {
        if !GeoCoordinates::is_valid_latitude(latitude)
            || !GeoCoordinates::is_valid_longitude(longitude) {
            return Result::Err(());
        }
        Result::Ok(GeoCoordinates {
            latitude,
            longitude
        })
    }

    fn parse(opt: Option<&&str>) -> Result<f64, ()> {
        match opt {
            Some(val) => {
                match val.parse::<f64>() {
                    Ok(float_value) => Result::Ok(float_value),
                    Err(_) => Result::Err(())
                }
            },
            None => Result::Err(())
        }
    }

    fn is_valid_latitude(latitude: f64) -> bool {
        latitude >= -90.0 && latitude <= 90.0
    }

    fn is_valid_longitude(longitude: f64) -> bool {
        longitude >= -180.0 && longitude <= 180.0
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