use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use buttercup_values::ValueHolder;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct VariableName {

    value: String

}

impl VariableName {

    pub fn new(value: String) -> VariableName {
        VariableName {
            value
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

}

impl From<String> for VariableName {
    fn from(value: String) -> Self {
        VariableName::new(value)
    }
}

pub trait VariableService {

    fn get_variable_value_by_name(&self,
                                  name: &VariableName)
                                  -> Result<Option<ValueHolder>, VariableValueAccessError>;

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Clone)]
pub enum VariableSpecification<T: TryFrom<ValueHolder> + Copy> {

    Literal(Arc<T>),
    VariableName(VariableName)

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct VariableServiceErrorReport {

    message: String,
    reason: String

}

impl VariableServiceErrorReport {

    pub fn new(message: String,
               reason: String) -> VariableServiceErrorReport {
        VariableServiceErrorReport {
            message,
            reason
        }
    }

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum VariableValueAccessError {

    VariableServiceError(VariableServiceErrorReport),
    ValueHolderConversionError,
    VariableOfGivenNameNotFound(VariableName),

}

impl<T: TryFrom<ValueHolder> + Copy> From<T> for VariableSpecification<T> {
    fn from(value: T) -> Self {
        VariableSpecification::Literal(Arc::new(value))
    }
}

impl<T: TryFrom<ValueHolder> + Copy> From<VariableName> for VariableSpecification<T> {
    fn from(variable_name: VariableName) -> Self {
        VariableSpecification::VariableName(variable_name)
    }
}

impl<T: TryFrom<ValueHolder> + Copy> VariableSpecification<T> {

    pub fn get_value<S: VariableService>(&self,
                                         service: &S)
                                         -> Result<Arc<T>, VariableValueAccessError> {
        match self {
            VariableSpecification::Literal(value) =>
                Result::Ok(value.clone()),
            VariableSpecification::VariableName(variable_name) =>
                match service.get_variable_value_by_name(variable_name) {
                    Ok(variable_holder_opt) =>
                        VariableSpecification::try_get_value_from(
                            variable_holder_opt,
                            variable_name),
                    Err(err) => Result::Err(err)
                }
        }
    }

    fn try_get_value_from(value_holder_opt: Option<ValueHolder>,
                          variable_name: &VariableName) -> Result<Arc<T>, VariableValueAccessError> {
        match value_holder_opt {
            None =>
                Result::Err(
                    VariableValueAccessError::VariableOfGivenNameNotFound(
                        variable_name.clone())),
            Some(value_holder) => {
                let result: Result<T, _> = value_holder.try_into();
                match result {
                    Err(_) =>
                        Result::Err(
                            VariableValueAccessError::ValueHolderConversionError),
                    Ok(value) =>
                        Result::Ok(Arc::new(value))
                }
            }
        }
    }

}


