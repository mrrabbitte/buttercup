use actix::{Actor, Message};
use actix::dev::{MessageResponse, ResponseChannel};
use serde::{Deserialize, Serialize};

use crate::app::agents::core::Agent;

#[derive(Message)]
#[rtype(result = "Result<TickStatus, TickError>")]
pub struct Tick;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickStatus {

    Success,
    Failure,
    Running

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickError {

    MissingBehaviorTree

}

