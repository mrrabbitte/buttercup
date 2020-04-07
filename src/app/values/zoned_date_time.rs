use std::str::FromStr;

use chrono::{NaiveDateTime, ParseError};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde::export::TryFrom;
use serde_json::Value;

use crate::app::values::wrappers::{TzWrapper, Wrapper};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub struct ZonedDateTime {

    date_time: NaiveDateTime,
    zone: TzWrapper

}

impl ZonedDateTime {

    pub fn new(date_time: NaiveDateTime,
               zone: Tz) -> ZonedDateTime {
        ZonedDateTime {
            date_time,
            zone: TzWrapper::new(zone)
        }
    }

    pub fn get_date_time(&self) -> &NaiveDateTime {
        &self.date_time
    }

    pub fn get_zone(&self) -> &Tz {
        &self.zone.get()
    }

}

impl TryFrom<&Value> for ZonedDateTime {
    type Error = ZonedDateTimeParsingError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        return match value {
            Value::Object(obj_val) => {
                match obj_val.get("date_time").and_then(Value::as_str) {
                    Some(str_val) => {
                        match str_val.parse::<NaiveDateTime>() {
                            Ok(date_time) =>
                                match obj_val.get("zone").and_then(Value::as_str) {
                                    Some(str_val) => match str_val.parse::<Tz>() {
                                        Ok(tz) =>
                                            Result::Ok(ZonedDateTime::new(date_time, tz)),
                                        Err(err) => Result::Err(
                                            ZonedDateTimeParsingError::InvalidTimeZone(err)),
                                    },
                                    None => Result::Err(ZonedDateTimeParsingError::InvalidFormat)
                                },
                            Err(err) =>
                                Result::Err(
                                    ZonedDateTimeParsingError::InvalidDateTime(
                                        String::from(str_val), err)),
                        }
                    },
                    None => Result::Err(ZonedDateTimeParsingError::InvalidFormat)
                }
            },
            _ => Result::Err(ZonedDateTimeParsingError::InvalidFormat)
        }
    }
}


#[derive(Debug)]
pub enum ZonedDateTimeParsingError {

    InvalidFormat,
    InvalidDateTime(String, ParseError),
    InvalidTimeZone(String)

}

impl FromStr for ZonedDateTime {
    type Err = ZonedDateTimeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<&str> = s
            .split("[")
            .collect();
        if s.len() < 2 {
            return Result::Err(ZonedDateTimeParsingError::InvalidFormat);
        }
        return match parsed.get(0) {
            Some(date_time_str_val) =>
                match date_time_str_val.parse::<NaiveDateTime>() {
                    Ok(date_time) =>
                        match parsed.get(1) {
                            Some(tz_str_val) =>
                                match tz_str_val.replace("]", "").parse::<Tz>() {
                                    Ok(tz) =>
                                        Result::Ok(
                                            ZonedDateTime::new(date_time, tz)),
                                    Err(err) =>
                                        Result::Err(
                                            ZonedDateTimeParsingError::InvalidTimeZone(err)),
                                },
                            None => Result::Err(ZonedDateTimeParsingError::InvalidFormat)
                        },
                    Err(err) =>
                        Result::Err(ZonedDateTimeParsingError::InvalidDateTime(
                            String::from(*date_time_str_val), err)),
                },
            None => Result::Err(ZonedDateTimeParsingError::InvalidFormat),
        };
    }

}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use chrono::{NaiveDate, NaiveTime};

    use super::*;

    #[test]
    fn test_from_str() {
        let zdt_result =
            "2020-03-18T12:33:34[Europe/Paris]".parse::<ZonedDateTime>();
        let zdt = zdt_result.unwrap();
        assert_eq!(NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(12, 33, 34)),
                   zdt.date_time);
        assert_eq!(Tz::Europe__Paris, *zdt.zone.get());
    }

    #[test]
    fn test_try_from_value() {
        let zdt_result: Result<ZonedDateTime, ZonedDateTimeParsingError> =
            (&Value::from_str(r#"
                {
                "date_time" : "2020-03-18T12:33:35",
                "zone" : "Africa/Cairo"
                }
                "#)
                .unwrap())
                .try_into();
        let zdt = zdt_result.unwrap();
        assert_eq!(NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 3, 18),
            NaiveTime::from_hms(12, 33, 35)),
                   zdt.date_time);
        assert_eq!(Tz::Africa__Cairo, *zdt.zone.get());
    }

}