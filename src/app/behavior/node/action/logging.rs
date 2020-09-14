use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

pub struct PrintLogActionNode;

impl BehaviorTreeNode for PrintLogActionNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        println!("I'm here!");
        Result::Ok(TickStatus::Success)
    }

}