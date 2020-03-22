use crate::transformations::astro::{IsAfterSunset, IsBeforeSunrise, IsDay};
use crate::transformations::date_time::{DayOfWeekFromDateTimeRetrieval, LocalToZonedDateTime};
use crate::transformations::geolocation::FindTimeZoneFromGeoCoordinates;
use crate::values::{ValueHolder, ValueType};
use crate::values::geolocation::GeoCoordinates;

pub mod date_time;
pub mod astro;
pub mod geolocation;

pub enum InputOrder {

    First,
    Second

}

pub enum TransformationError {

    InvalidInputType(ValueHolder, InputOrder),
    CouldNotFindTimezone(GeoCoordinates),
    UnknownTimezone(String)

}

pub enum SingleInputTransformation {

    DayOfWeekFromDateTimeRetrieval(DayOfWeekFromDateTimeRetrieval),
    FindTimeZoneFromGeoCoordinates(FindTimeZoneFromGeoCoordinates)

}

impl SingleInputTransformation {

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        self.get_transformer().transform(value)
    }

    pub fn get_input_types(&self) -> &[ValueType] {
        self.get_transformer().get_input_types()
    }

    pub fn is_input_type_ok(&self,
                            input_type: &ValueType) -> bool {
        self.get_input_types().contains(input_type)
    }

    fn get_transformer(&self) -> &dyn SingleInputTransformer {
        return match self {
            SingleInputTransformation::DayOfWeekFromDateTimeRetrieval(t) => t,
            SingleInputTransformation::FindTimeZoneFromGeoCoordinates(t) => t
        };
    }

}

pub trait SingleInputTransformer {

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;

}

pub enum DoubleInputTransformation {

    IsAfterSunset(IsAfterSunset),
    IsBeforeSunrise(IsBeforeSunrise),
    IsDay(IsDay),
    LocalToZonedDateTime(LocalToZonedDateTime)

}

impl DoubleInputTransformation {

    pub fn transform(&self,
                     first: &ValueHolder,
                     second: &ValueHolder) -> Result<ValueHolder, TransformationError> {
        self.get_transformer().transform(first, second)
    }

    pub fn get_first_input_types(&self) -> &'static [ValueType] {
        self.get_transformer().get_first_input_types()
    }

    pub fn get_second_input_types(&self) -> &'static [ValueType] {
        self.get_transformer().get_second_input_types()
    }

    pub fn is_first_input_type_ok(&self,
                                  input_type: &ValueType) -> bool {
        self.get_first_input_types().contains(input_type)
    }

    pub fn is_second_input_type_ok(&self,
                                   input_type: &ValueType) -> bool {
        self.get_second_input_types().contains(input_type)
    }

    fn get_transformer(&self) -> &dyn DoubleInputTransformer {
        return match self {
            DoubleInputTransformation::IsAfterSunset(t) => t,
            DoubleInputTransformation::IsBeforeSunrise(t) => t,
            DoubleInputTransformation::IsDay(t) => t,
            DoubleInputTransformation::LocalToZonedDateTime(t) => t
        };
    }

}

pub trait DoubleInputTransformer {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_first_input_types(&self) -> &'static [ValueType];
    fn get_second_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;

}