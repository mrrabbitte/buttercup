use crate::values::geolocation::GeoCoordinates;
use crate::values::ValueHolder;

pub mod mono;
pub mod bi;

pub enum InputOrder {

    First,
    Second

}

pub enum TransformationError {

    InvalidInputType(ValueHolder, InputOrder),
    CouldNotFindTimezone(GeoCoordinates),
    UnknownTimezone(String)

}

