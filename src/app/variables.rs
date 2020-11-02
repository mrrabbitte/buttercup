use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use crate::app::values::{ValueHolder, ValuesPayload, ValueType};
use std::rc::Rc;

pub type VariableName = String;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum VariableSpecification<T: TryFrom<ValueHolder> + Copy> {

    Literal(Rc<T>),
    VariableName(VariableName)

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum VariableValueAccessError {

    VariableOfGivenNameNotFound(VariableName),
    ValueHolderConversionError

}

impl<T: TryFrom<ValueHolder> + Copy> VariableSpecification<T> {

    pub fn get_value(&self,
                     payload: &ValuesPayload) -> Result<Rc<T>, VariableValueAccessError> {
        match self {
            VariableSpecification::Literal(value) =>
                Result::Ok(value.clone()),
            VariableSpecification::VariableName(variable_name) =>
                match payload.get(variable_name) {
                    None =>
                        Result::Err(
                            VariableValueAccessError::VariableOfGivenNameNotFound(
                                variable_name.clone())
                        ),
                    Some(value_holder) =>
                        {
                            let result: Result<T, _> =
                                value_holder
                                    .clone()
                                    .try_into();
                            match result {
                                Err(_) =>
                                    Result::Err(
                                        VariableValueAccessError::ValueHolderConversionError),
                                Ok(value) =>
                                    Result::Ok(Rc::new(value))
                            }
                        }
                }
        }
    }

}


