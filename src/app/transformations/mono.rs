use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::app::transformations::mono::day_of_week::DayOfWeekFromDateTimeRetrieval;
use crate::app::transformations::mono::geolocation::FindTimeZoneFromGeoCoordinates;
use crate::app::transformations::transformer::TransformationError;
use crate::app::values::{ValueHolder, ValueType};

pub mod day_of_week;
pub mod geolocation;

use serde::{Serialize, Deserialize};

#[derive(EnumIter, Serialize, Deserialize)]
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

    pub fn initialize() {
        for transformation in MonoInputTransformation::iter() {
            transformation.get_transformer().initialize();
        }
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

    fn initialize(&self) {}
    fn transform(&self,
                 value: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;
    
}