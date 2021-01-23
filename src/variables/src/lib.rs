use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::blackboards::service::BlackboardError;
use crate::app::values::{ValueHolder, ValuesPayload, ValueType};

pub type VariableName = String;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum VariableSpecification<T: TryFrom<ValueHolder> + Copy> {

    Literal(Arc<T>),
    VariableName(VariableName)

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum VariableValueAccessError {

    BlackboardError(BlackboardError),
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

    pub fn get_value(&self,
                     context: &BTNodeExecutionContext) -> Result<Arc<T>, VariableValueAccessError> {
        match self {
            VariableSpecification::Literal(value) =>
                Result::Ok(value.clone()),
            VariableSpecification::VariableName(variable_name) =>
                match context.get_value(variable_name) {
                    Ok(variable_holder_opt) =>
                        VariableSpecification::try_get_value_from(
                            variable_holder_opt,
                            variable_name),
                    Err(err) =>
                        Result::Err(VariableValueAccessError::BlackboardError(err))
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
                let result: Result<T, _> =
                    value_holder
                        .clone()
                        .try_into();
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


