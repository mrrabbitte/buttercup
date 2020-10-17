use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix::Message;
use serde::{Deserialize, Serialize};

use crate::app::behavior::node::BTNodeAddress;
use crate::app::blackboards::service::BlackboardError;

#[derive(Message)]
#[rtype(result = "Result<TickStatus, TickError>")]
pub struct Tick;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickStatus {

    Success,
    Failure

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickError {

    BlackboardError(i32, BlackboardError),
    CompositeError(i32, Vec<(i32, TickError)>),

}

impl TickError {

    pub fn get_node_id(&self) -> &i32 {
        match self {
            TickError::BlackboardError(id, _) => id,
            TickError::CompositeError(id, _) => id
        }
    }

}



