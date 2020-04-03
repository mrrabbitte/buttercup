use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::app::values::geolocation::GeoCoordinates;

#[derive(Debug)]
pub struct SunPositionTimes {

    date_time_at_local: NaiveDateTime,
    sunset_at_local: NaiveDateTime,
    sunrise_at_local: NaiveDateTime

}

impl SunPositionTimes {

    pub fn new(date_time_at_local: &NaiveDateTime,
               time_zone: &Tz,
               coordinates: &GeoCoordinates) -> SunPositionTimes {
        let (sunrise_ts, sunset_ts) = sunrise::sunrise_sunset(
            *coordinates.get_latitude(),
            *coordinates.get_longitude(),
            date_time_at_local.year(),
            date_time_at_local.month(),
            date_time_at_local.day(),
        );
        SunPositionTimes {
            date_time_at_local: date_time_at_local.clone(),
            sunset_at_local: time_zone.from_utc_datetime(
                &NaiveDateTime::from_timestamp(sunset_ts, 0))
                .naive_local(),
            sunrise_at_local: time_zone.from_utc_datetime(
                &NaiveDateTime::from_timestamp(sunrise_ts, 0))
                .naive_local()
        }
    }

    pub fn is_after_sunset(&self) -> bool {
        self.sunset_at_local.lt(&self.date_time_at_local)
    }

    pub fn is_before_sunrise(&self) -> bool {
        self.sunrise_at_local.gt(&self.date_time_at_local)
    }

    pub fn is_day(&self) -> bool {
        !self.is_before_sunrise() && !self.is_after_sunset()
    }

}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::*;

    fn sun_position_times(dt_at_local: &NaiveDateTime) -> SunPositionTimes {
        let tz: Tz = "Europe/Warsaw".parse().unwrap();
        let coordinates = "53.01375,18.59814".parse::<GeoCoordinates>().unwrap();
        SunPositionTimes::new(&dt_at_local, &tz, &coordinates)
    }

    #[test]
    fn test_simple_day() {
        let day_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(15, 28, 33));
        let sun_position_times = sun_position_times(&day_at_local);
        assert_eq!(false, sun_position_times.is_after_sunset());
        assert_eq!(false, sun_position_times.is_before_sunrise());
        assert_eq!(true, sun_position_times.is_day());
    }

    #[test]
    fn test_simple_night() {
        let night_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(20, 28, 33));
        let sun_position_times = sun_position_times(&night_at_local);
        assert_eq!(true, sun_position_times.is_after_sunset());
        assert_eq!(false, sun_position_times.is_before_sunrise());
        assert_eq!(false, sun_position_times.is_day());
    }

    #[test]
    fn test_simple_before_sunrise() {
        let before_sunrise_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(3, 28, 33));
        let sun_position_times =
            sun_position_times(&before_sunrise_at_local);
        assert_eq!(false,
                   sun_position_times.is_after_sunset());
        assert_eq!(true,
                   sun_position_times.is_before_sunrise());
        assert_eq!(false, sun_position_times.is_day());
    }

}