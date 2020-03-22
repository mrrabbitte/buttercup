use chrono_tz::Tz;

use crate::transformations::{InputOrder, TransformationError};
use crate::transformations::mono::MonoInputTransformer;
use crate::values::{ValueHolder, ValueType};
use crate::values::geolocation::GeoCoordinates;

pub struct FindTimeZoneFromGeoCoordinates;

const INPUT_TYPE: [ValueType; 1] = [ValueType::GeoCoordinates];
const RESULT_TYPE: ValueType = ValueType::TimeZone;

impl MonoInputTransformer for FindTimeZoneFromGeoCoordinates {

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

    fn find_time_zone(coordinates: &GeoCoordinates) -> Result<ValueHolder, TransformationError> {
        return match tz_search::lookup(
            *coordinates.get_latitude(), *coordinates.get_longitude()) {
            Some(tz_str) =>
                match tz_str.parse::<Tz>() {
                    Ok(tz) => Result::Ok(ValueHolder::TimeZone(tz)),
                    Err(_) => Result::Err(TransformationError::UnknownTimezone(tz_str)),
                },
            None => Result::Err(TransformationError::CouldNotFindTimezone(coordinates.clone()))
        };
    }

}