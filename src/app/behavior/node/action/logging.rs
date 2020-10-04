use crate::app::behavior::node::{BehaviorTreeNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::context::BTNodeExecutionContext;

pub struct PrintLogActionNode;

impl BehaviorTreeNode for PrintLogActionNode {

    fn tick(&mut self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        println!("I'm here!");
        Result::Ok(TickStatus::Success)
    }

}