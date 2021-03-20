use std::sync::Arc;
use std::thread;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::ActionBTNode;
use crate::tick::{TickError, TickHeader, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PrintLogActionNode {

    id: i32,
    message: String

}

impl PrintLogActionNode {

    pub fn new(id: i32, message: String) -> PrintLogActionNode {
        PrintLogActionNode { id, message }
    }

}

#[async_trait]
impl BehaviorTreeNode for PrintLogActionNode {

    async fn do_tick(&self,
                     _: &TickHeader,
                     _: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        println!("[{}] {}", self.id, self.message);
        Result::Ok(TickStatus::Success)
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<PrintLogActionNode> for BTNode {
    fn from(node: PrintLogActionNode) -> Self {
        BTNode::Action(ActionBTNode::PrintLog(node))
    }
}