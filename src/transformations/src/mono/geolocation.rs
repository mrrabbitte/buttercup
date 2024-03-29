use buttercup_values::{ValueHolder, ValueType};
use buttercup_values::geolocation::GeoCoordinates;
use buttercup_values::wrappers::{TzWrapper, Wrapper};
use chrono_tz::Tz;

use crate::mono::MonoInputTransformer;
use crate::transformer::{InputOrder, TransformationError};

pub struct FindTimeZoneFromGeoCoordinates;

const INPUT_TYPE: [ValueType; 1] = [ValueType::GeoCoordinates];
const RESULT_TYPE: ValueType = ValueType::TimeZone;

impl MonoInputTransformer for FindTimeZoneFromGeoCoordinates {

    fn initialize(&self) {
        match tz_search::lookup(53.350140, -6.266155) {
            None => panic!("Could not initialize time zone search."),
            Some(found_zone) => assert_eq!("Europe/Dublin", found_zone),
        }
    }

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        match value {
            ValueHolder::GeoCoordinates(coordinates) =>
                FindTimeZoneFromGeoCoordinates::find_time_zone(&coordinates),
            _ => Result::Err(
                TransformationError::InvalidInputType(value.clone(), InputOrder::First))
        }
    }

    fn get_input_types(&self) -> &'static [ValueType] {
        &INPUT_TYPE
    }

    fn get_result_type(&self) -> &'static ValueType {
        &RESULT_TYPE
    }

}

impl FindTimeZoneFromGeoCoordinates {

    const INSTANCE: FindTimeZoneFromGeoCoordinates = FindTimeZoneFromGeoCoordinates{};

    pub fn instance() -> &'static FindTimeZoneFromGeoCoordinates {
        &FindTimeZoneFromGeoCoordinates::INSTANCE
    }

    fn find_time_zone(coordinates: &GeoCoordinates) -> Result<ValueHolder, TransformationError> {
        return match tz_search::lookup(
            coordinates.get_latitude_as_f64(), coordinates.get_longitude_as_f64()) {
            Some(tz_str) =>
                match tz_str.parse::<Tz>() {
                    Ok(tz) =>
                        Result::Ok(
                            ValueHolder::TimeZone(
                                TzWrapper::new(tz))),
                    Err(_) => Result::Err(TransformationError::UnknownTimezone(tz_str)),
                },
            None => Result::Err(TransformationError::CouldNotFindTimezone(coordinates.clone()))
        };
    }

}