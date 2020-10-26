use std::future::Future;

use actix::{Actor, Context, Handler, ResponseActFuture};
use uuid::Uuid;

use crate::app::behavior::tick::{TickError, TickStatus};
use crate::app::behavior::tree::BehaviorTree;

pub struct Agent {

    address: AgentAddress,
    tree: BehaviorTree,
    uuid: Uuid

}

impl Agent {

    pub fn new(address: AgentAddress,
               tree: BehaviorTree) -> Agent {
        Agent {
            address,
            tree,
            uuid: Uuid::new_v4()
        }
    }

    pub(crate) async fn tick(&self) -> Result<TickStatus, TickError> {
        println!("Performing tick: {}", self.uuid);
        self.tree.tick().await
    }
}

pub struct AgentAddress {

    id: i32,
    index: u32

}

impl AgentAddress {

    pub fn new(id: i32,
               index: u32) -> AgentAddress {
        AgentAddress {
            id,
            index
        }
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::app::address::Address;
    use crate::app::behavior::context::BTNodeExecutionContext;
    use crate::app::behavior::node::action::logging::PrintLogActionNode;
    use crate::app::behavior::node::BTNodeAddress;
    use crate::app::blackboards::service::BlackboardService;

    use super::*;

    #[actix_rt::test]
    async fn test_returns_status() {
        assert_eq!(Agent::new(AgentAddress {id: 1, index: 1},
                              BehaviorTree::new(1,
                                                Arc::new(BTNodeExecutionContext::new(
                                                    Uuid::from_u128(1),
                                                    Arc::new(
                                                        BlackboardService::new(
                                                            DashMap::new())))),
                                                PrintLogActionNode::new(
                                                    1, "hello".to_owned())
                                                    .into()))
            .tick().await, Result::Ok(TickStatus::Success));
    }

}
