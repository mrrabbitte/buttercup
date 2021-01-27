use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix::Message;
use serde::{Deserialize, Serialize};

use crate::context::reactive::ReactiveServiceError;
use buttercup_blackboards::BlackboardError;
use buttercup_variables::VariableValueAccessError;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickStatus {

    Success,
    Failure

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickError {

    AbortedExecution(i32),
    BlackboardError(i32, BlackboardError),
    CompositeError(i32, Vec<(i32, TickError)>),
    ReactiveServiceError(i32, ReactiveServiceError),
    VariableValueAccessError(i32, VariableValueAccessError)

}

impl TickError {

    pub fn get_node_id(&self) -> &i32 {
        match self {
            TickError::AbortedExecution(id) => id,
            TickError::BlackboardError(id, _) => id,
            TickError::CompositeError(id, _) => id,
            TickError::ReactiveServiceError(id, _) => id,
            TickError::VariableValueAccessError(id, _) => id
        }
    }

}



