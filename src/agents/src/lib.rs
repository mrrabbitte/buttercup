use std::future::Future;

use actix::{Actor, Context, Handler, ResponseActFuture};
use buttercup_bts::tick::{TickError, TickStatus};
use buttercup_bts::tree::BehaviorTree;
use uuid::Uuid;

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

    pub async fn tick(&self) -> Result<TickStatus, TickError> {
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

    use buttercup_blackboards::BlackboardService;
    use buttercup_bts::context::BTNodeExecutionContext;
    use buttercup_bts::node::action::logging::PrintLogActionNode;
    use dashmap::DashMap;

    use super::*;

    #[actix_rt::test]
    async fn test_returns_status() {
        assert_eq!(Agent::new(AgentAddress {id: 1, index: 1},
                              BehaviorTree::new(1,
                                                Arc::new(Default::default()),
                                                PrintLogActionNode::new(
                                                    1, "hello".to_owned())
                                                    .into()))
                       .tick().await, Result::Ok(TickStatus::Success));
    }

}
