use crate::transformations::bi::astro::{IsAfterSunset, IsBeforeSunrise, IsDay};
use crate::transformations::bi::local_to_zoned::LocalToZonedDateTime;
use crate::transformations::TransformationError;
use crate::values::{ValueHolder, ValueType};

pub mod astro;
pub mod local_to_zoned;

pub enum BiInputTransformation {

    IsAfterSunset(IsAfterSunset),
    IsBeforeSunrise(IsBeforeSunrise),
    IsDay(IsDay),
    LocalToZonedDateTime(LocalToZonedDateTime)

}

impl BiInputTransformation {

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

    fn get_transformer(&self) -> &dyn BiInputTransformer {
        return match self {
            BiInputTransformation::IsAfterSunset(t) => t,
            BiInputTransformation::IsBeforeSunrise(t) => t,
            BiInputTransformation::IsDay(t) => t,
            BiInputTransformation::LocalToZonedDateTime(t) => t
        };
    }

}

pub trait BiInputTransformer {

    fn transform(&self,
                 first: &ValueHolder,
                 second: &ValueHolder) -> Result<ValueHolder, TransformationError>;
    fn get_first_input_types(&self) -> &'static [ValueType];
    fn get_second_input_types(&self) -> &'static [ValueType];
    fn get_result_type(&self) -> &'static ValueType;

}