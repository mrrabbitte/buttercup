use std::sync::Arc;

use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::node::action::ActionBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use std::thread;

pub struct PrintLogActionNode {

    id: i32,
    message: String

}

impl PrintLogActionNode {

    pub fn new(id: i32, message: String) -> PrintLogActionNode {
        PrintLogActionNode { id, message }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for PrintLogActionNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        println!("[{}] {}", self.id, self.message);
        Result::Ok(TickStatus::Success)
    }
}

impl From<PrintLogActionNode> for BTNode {
    fn from(node: PrintLogActionNode) -> Self {
        BTNode::Action(ActionBTNode::PrintLog(node))
    }
}