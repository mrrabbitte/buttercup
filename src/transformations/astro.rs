use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::transformations::{DoubleValueTransformer, InputOrder, TransformationError};
use crate::transformations::astro::sun_position::SunPositionTimes;
use crate::values::{ValueHolder, ValueType};

mod sun_position;

const SUN_POSITION_FIRST_INPUT: [ValueType; 1] = [ValueType::ZonedDateTime];

const SUN_POSITION_SECOND_INPUT: [ValueType; 1] = [ValueType::GeoCoordinates];

struct Astro;

impl Astro {

    fn is_first_input_type_ok(value_type: &ValueType) -> bool {
        ValueType::ZonedDateTime == value_type
    }

    fn is_second_input_type_ok(value_type: &ValueType) -> bool {
        ValueType::GeoCoordinates == value_type
    }

    fn transform(first: ValueHolder,
                 second: ValueHolder) -> Result<SunPositionTimes, TransformationError> {
        return match first {
            ValueHolder::ZonedDateTime(zdt) =>
                match second {
                    ValueHolder::GeoCoordinates(coordinates) =>
                        Result::Ok(
                            SunPositionTimes::new(
                                zdt.get_date_time(),
                                zdt.get_zone(),
                                &coordinates)),
                    _ => Result::Err(
                        TransformationError::InvalidInputType(second, InputOrder::Second))
                },
            _ => Result::Err(TransformationError::InvalidInputType(first, InputOrder::First))
        };
    }

}

pub struct IsAfterSunset;

impl DoubleValueTransformer for IsAfterSunset {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_after_sunset())),
            Err(err) => Result::Err(err),
        }
    }

    fn is_first_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_first_input_type_ok(value_type)
    }

    fn is_second_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_second_input_type_ok(value_type)
    }

    fn get_result_type() -> ValueType {
        ValueType::Boolean
    }
}

pub struct IsBeforeSunrise;

impl DoubleValueTransformer for IsBeforeSunrise {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_before_sunrise())),
            Err(err) => Result::Err(err),
        }
    }

    fn is_first_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_first_input_type_ok(value_type)
    }

    fn is_second_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_second_input_type_ok(value_type)
    }

    fn get_result_type() -> ValueType {
        ValueType::Boolean
    }
}

pub struct IsDay;

impl DoubleValueTransformer for IsDay {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_day())),
            Err(err) => Result::Err(err),
        }
    }

    fn is_first_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_first_input_type_ok(value_type)
    }

    fn is_second_input_value_type_ok(value_type: &ValueType) -> bool {
        Astro::is_second_input_type_ok(value_type)
    }

    fn get_result_type() -> ValueType {
        ValueType::Boolean
    }
}