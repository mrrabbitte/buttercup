use std::convert::{TryFrom, TryInto};

use crate::app::values::{ValueHolder, ValuesPayload, ValueType};

pub type VariableName = String;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum VariableSpecification<T: TryFrom<ValueHolder>> {

    Literal(T),
    VariableName(VariableName)

}

pub enum VariableValueAccessError {

    VariableOfGivenNameNotFound(VariableName),
    ValueHolderConversionError

}


