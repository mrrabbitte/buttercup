use actix::Message;

use serde::{Deserialize, Serialize};
use crate::app::blackboards::service::BlackboardError;
use crate::app::behavior::node::BTNodeAddress;

#[derive(Message)]
#[rtype(result = "Result<TickStatus, TickError>")]
pub struct Tick;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickStatus {

    Success,
    Failure,
    Running(BTNodeAddress)

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickError {

    BlackboardError(BlackboardError),
    MissingBehaviorTreeNode

}

