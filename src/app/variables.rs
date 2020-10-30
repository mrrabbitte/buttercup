use crate::app::values::{ValuesPayload, ValueHolder};
use std::convert::{TryInto, TryFrom};

type VariableName = String;

pub enum VariableSpecification<T: TryFrom<ValueHolder>> {

    Literal(T),
    VariableName(VariableName)

}

pub enum VariableValueAccessError {

    ValueHolderTypeMismatch,
    VariableValueNotFound

}

impl<T: TryFrom<ValueHolder>> VariableSpecification<T> {

    pub fn get_value(&self,
                     payload: &ValuesPayload) -> Result<&T, VariableValueAccessError> {
        match self {
            VariableSpecification::Literal(value) =>
                Result::Ok(value),
            VariableSpecification::VariableName(variable_name) =>
                match payload.get(variable_name) {
                    None => Result::Err(VariableValueAccessError::VariableValueNotFound),
                    Some(value_holder) =>
                        match value_holder.try_into() {
                            Ok(value) => Result::Ok(value),
                            Err(_) => Result::Err(VariableValueAccessError::ValueHolderTypeMismatch)
                        }
                }
        }
    }

}