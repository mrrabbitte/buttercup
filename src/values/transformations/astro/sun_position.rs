use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::values::geolocation::GeoCoordinates;

#[derive(Debug)]
struct SunPositionTimes {

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
            sunset_at_local: time_zone.from_utc_datetime(
                &NaiveDateTime::from_timestamp(sunset_ts, 0))
                .naive_local(),
            sunrise_at_local: time_zone.from_utc_datetime(
                &NaiveDateTime::from_timestamp(sunrise_ts, 0))
                .naive_local()
        }
    }

    pub fn is_after_sunset(&self,
                           local_date_time: &NaiveDateTime) -> bool {
        self.sunset_at_local.lt(local_date_time)
    }

    pub fn is_before_sunrise(&self,
                             local_date_time: &NaiveDateTime) -> bool {
        self.sunrise_at_local.gt(local_date_time)
    }

    pub fn is_day(&self,
                  local_date_time: &NaiveDateTime) -> bool {
        !self.is_before_sunrise(local_date_time) && !self.is_after_sunset(local_date_time)
    }

}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::*;

    #[test]
    fn test_simple() {
        let tz: Tz = "Europe/Warsaw".parse().unwrap();
        let day_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(15, 28, 33));
        let coordinates = "53.01375,18.59814".parse::<GeoCoordinates>().unwrap();
        let sun_position_times =
            SunPositionTimes::new(&day_at_local, &tz, &coordinates);
        assert_eq!(false, sun_position_times.is_after_sunset(&day_at_local));
        assert_eq!(false, sun_position_times.is_before_sunrise(&day_at_local));
        assert_eq!(true, sun_position_times.is_day(&day_at_local));
        let night_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(20, 28, 33));
        assert_eq!(true, sun_position_times.is_after_sunset(&night_at_local));
        assert_eq!(false, sun_position_times.is_before_sunrise(&night_at_local));
        assert_eq!(false, sun_position_times.is_day(&night_at_local));
        let before_sunrise_at_local = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(3, 28, 33));
        assert_eq!(false,
                   sun_position_times.is_after_sunset(&before_sunrise_at_local));
        assert_eq!(true,
                   sun_position_times.is_before_sunrise(&before_sunrise_at_local));
        assert_eq!(false, sun_position_times.is_day(&before_sunrise_at_local));
    }

}