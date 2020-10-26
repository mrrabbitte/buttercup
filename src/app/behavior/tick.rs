use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix::Message;
use serde::{Deserialize, Serialize};

use crate::app::behavior::context::reactive::ReactiveServiceError;
use crate::app::behavior::node::BTNodeAddress;
use crate::app::blackboards::service::BlackboardError;

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

}

impl TickError {

    pub fn get_node_id(&self) -> &i32 {
        match self {
            TickError::AbortedExecution(id) => id,
            TickError::BlackboardError(id, _) => id,
            TickError::CompositeError(id, _) => id,
            TickError::ReactiveServiceError(id, _) => id
        }
    }

}



