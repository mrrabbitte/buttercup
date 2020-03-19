use crate::transformations::{InputOrder, SingleValueTransformer, TransformationError};
use crate::values::{ValueHolder, ValueType};
use crate::values::geolocation::GeoCoordinates;

pub struct FindTimeZoneFromGeoCoordinates;

impl SingleValueTransformer for FindTimeZoneFromGeoCoordinates {

    fn transform(value: ValueHolder) -> Result<ValueHolder, TransformationError> {
        match value {
            ValueHolder::GeoCoordinates(coordinates) =>
                FindTimeZoneFromGeoCoordinates::find_time_zone(&coordinates),
            _ => Result::Err(TransformationError::InvalidInputType(value, InputOrder::First))
        }
    }

    fn get_input_value_type() -> &'static [ValueType] {
        &[ValueType::GeoCoordinates]
    }

    fn get_result_value_type() -> ValueType {
        ValueType::TimeZone
    }

}

impl FindTimeZoneFromGeoCoordinates {

    fn find_time_zone(coordinates: &GeoCoordinates) -> Result<ValueHolder, TransformationError> {
        return match tz_search::lookup(
            *coordinates.get_latitude(), *coordinates.get_longitude()) {
            Some(_) => {},
            None =>
        }
    }

}