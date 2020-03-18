use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::transformations::{DoubleValueTransformation, InputOrder, TransformationError};
use crate::transformations::astro::sun_position::SunPositionTimes;
use crate::values::{ValueHolder, ValueType};

mod sun_position;

const SUN_POSITION_FIRST_INPUT: [ValueType; 1] = [ValueType::ZonedDateTime];

const SUN_POSITION_SECOND_INPUT: [ValueType; 1] = [ValueType::GeoCoordinates];

struct Astro;

impl Astro {

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

impl DoubleValueTransformation for IsAfterSunset {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_after_sunset())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_FIRST_INPUT
    }

    fn get_second_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_SECOND_INPUT
    }

    fn get_result_value_type() -> ValueType {
        ValueType::Boolean
    }

}

pub struct IsBeforeSunrise;

impl DoubleValueTransformation for IsBeforeSunrise {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_before_sunrise())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_FIRST_INPUT
    }

    fn get_second_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_SECOND_INPUT
    }

    fn get_result_value_type() -> ValueType {
        ValueType::Boolean
    }

}


pub struct IsDay;

impl DoubleValueTransformation for IsDay {

    fn transform(first: ValueHolder,
                 second: ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_day())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_FIRST_INPUT
    }

    fn get_second_input_value_type() -> &'static [ValueType] {
        &SUN_POSITION_SECOND_INPUT
    }

    fn get_result_value_type() -> ValueType {
        ValueType::Boolean
    }

}