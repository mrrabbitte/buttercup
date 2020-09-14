use actix::{Actor, Context, Handler, Message};

use crate::app::behavior::tick::{Tick, TickStatus, TickError};
use crate::app::behavior::tree::BehaviorTree;
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

    fn tick(&self) -> Result<TickStatus, TickError> {
        println!("Performing tick: {}", self.uuid);
        Result::Ok(TickStatus::Success)
    }
}

impl Actor for Agent {
    type Context = Context<Agent>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor: {} is alive", self.uuid);
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor: {} is stopped", self.uuid);
    }
}

impl Handler<Tick> for Agent {
    type Result = Result<TickStatus, TickError>;

    fn handle(&mut self, msg: Tick, ctx: &mut Context<Agent>) -> Self::Result {
        println!("Got a tick.");
        println!("{:?}", ctx);
        self.tick()
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
    use actix::System;

    use crate::app::address::Address;
    use crate::app::behavior::node::BTNodeAddress;

    use super::*;

    #[test]
    fn test_returns_status() {
        assert_eq!(Agent::new(AgentAddress {id: 1, index: 1},
                              BehaviorTree::new(1, BTNodeAddress::new(1, 1)))
            .tick(), Result::Ok(TickStatus::Success));
    }

}
