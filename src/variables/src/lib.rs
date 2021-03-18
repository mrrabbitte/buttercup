use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use buttercup_values::ValueHolder;
use crate::specification::VariableValueAccessError;

pub mod specification;

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
