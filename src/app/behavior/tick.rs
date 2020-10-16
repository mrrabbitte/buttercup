use actix::Message;

use serde::{Deserialize, Serialize};
use crate::app::blackboards::service::BlackboardError;
use crate::app::behavior::node::BTNodeAddress;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

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

    BlackboardError(BlackboardError),
    MissingBehaviorTreeNode

}

pub struct TickResponse {

}

impl Future<Output=Result<TickStatus, TickError>> for TickResponse {
    type Output = Result<TickStatus, TickError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}

