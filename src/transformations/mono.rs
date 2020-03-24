use crate::transformations::mono::day_of_week::DayOfWeekFromDateTimeRetrieval;
use crate::transformations::mono::geolocation::FindTimeZoneFromGeoCoordinates;
use crate::transformations::transformer::TransformationError;
use crate::values::{ValueHolder, ValueType};

pub mod day_of_week;
pub mod geolocation;

pub enum MonoInputTransformation {

    DayOfWeekFromDateTimeRetrieval,
    FindTimeZoneFromGeoCoordinates

}

impl MonoInputTransformation {

    pub fn transform(&self,
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

    fn get_transformer(&self) -> &dyn MonoInputTransformer {
        return match self {
            MonoInputTransformation::DayOfWeekFromDateTimeRetrieval
            => DayOfWeekFromDateTimeRetrieval::instance(),
            MonoInputTransformation::FindTimeZoneFromGeoCoordinates
            => FindTimeZoneFromGeoCoordinates::instance()
        };
    }

}

pub trait MonoInputTransformer {

    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;

}