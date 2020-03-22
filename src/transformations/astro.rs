use chrono::{Datelike, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::transformations::{DoubleInputTransformer, InputOrder, TransformationError};
use crate::transformations::astro::sun_position::SunPositionTimes;
use crate::values::{ValueHolder, ValueType};

mod sun_position;

const SUN_POSITION_FIRST_INPUT: [ValueType; 1] = [ValueType::ZonedDateTime];

const SUN_POSITION_SECOND_INPUT: [ValueType; 1] = [ValueType::GeoCoordinates];

const BOOL_RESULT_TYPE: ValueType = ValueType::Boolean;

struct Astro;

impl Astro {

    fn get_first_input_types() -> &'static [ValueType] {
        &SUN_POSITION_FIRST_INPUT
    }

    fn get_second_input_types() -> &'static [ValueType] {
        &SUN_POSITION_SECOND_INPUT
    }

    fn transform(first: &ValueHolder,
                 second: &ValueHolder) -> Result<SunPositionTimes, TransformationError> {
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
                        TransformationError::InvalidInputType(second.clone(), InputOrder::Second))
                },
            _ => Result::Err(
                TransformationError::InvalidInputType(first.clone(), InputOrder::First))
        };
    }

}

pub struct IsAfterSunset;

impl DoubleInputTransformer for IsAfterSunset {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_after_sunset())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_types(&self) -> &'static [ValueType] {
        Astro::get_first_input_types()
    }

    fn get_second_input_types(&self) -> &'static [ValueType] {
        Astro::get_second_input_types()
    }

    fn get_result_type(&self) -> &'static ValueType {
        &BOOL_RESULT_TYPE
    }
}

pub struct IsBeforeSunrise;

impl DoubleInputTransformer for IsBeforeSunrise {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_before_sunrise())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_types(&self) -> &'static [ValueType] {
        Astro::get_first_input_types()
    }

    fn get_second_input_types(&self) -> &'static [ValueType] {
        Astro::get_second_input_types()
    }

    fn get_result_type(&self) -> &'static ValueType {
        &BOOL_RESULT_TYPE
    }

}

pub struct IsDay;

impl DoubleInputTransformer for IsDay {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder)
                 -> Result<ValueHolder, TransformationError> {
        return match Astro::transform(first, second) {
            Ok(sun_position) => Result::Ok(
                ValueHolder::Boolean(sun_position.is_day())),
            Err(err) => Result::Err(err),
        }
    }

    fn get_first_input_types(&self) -> &'static [ValueType] {
        Astro::get_first_input_types()
    }

    fn get_second_input_types(&self) -> &'static [ValueType] {
        Astro::get_second_input_types()
    }

    fn get_result_type(&self) -> &'static ValueType {
        &BOOL_RESULT_TYPE
    }

}