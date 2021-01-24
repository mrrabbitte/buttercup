use buttercup_values::{ValueHolder, ValueType};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::di::astro::{IsAfterSunset, IsBeforeSunrise, IsDay};
use crate::di::local_to_zoned::LocalToZonedDateTime;
use crate::transformer::TransformationError;

pub mod astro;
pub mod local_to_zoned;

#[derive(EnumIter, Serialize, Deserialize)]
pub enum DiInputTransformation {

    IsAfterSunset,
    IsBeforeSunrise,
    IsDay,
    LocalToZonedDateTime

}

impl DiInputTransformation {

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

    pub fn initialize() {
        for transformation in DiInputTransformation::iter() {
            transformation.get_transformer().initialize();
        }
    }

    fn get_transformer(&self) -> &dyn DiInputTransformer {
        return match self {
            DiInputTransformation::IsAfterSunset => IsAfterSunset::instance(),
            DiInputTransformation::IsBeforeSunrise => IsBeforeSunrise::instance(),
            DiInputTransformation::IsDay => IsDay::instance(),
            DiInputTransformation::LocalToZonedDateTime => LocalToZonedDateTime::instance()
        };
    }

}

pub trait DiInputTransformer {

    fn initialize(&self) {}
    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_first_input_types(&self) -> &'static [ValueType];
    fn get_second_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;
    
}