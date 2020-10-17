use std::time::Duration;

use async_std::task;
use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::node::action::ActionBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct WaitDurationActionNode {

    id: i32,
    duration: Duration

}

impl WaitDurationActionNode {

    pub fn new(id: i32,
               duration: Duration) -> WaitDurationActionNode {
        WaitDurationActionNode { id, duration }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for WaitDurationActionNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        task::sleep(self.duration).await;
        Result::Ok(TickStatus::Success)
    }
}

impl From<WaitDurationActionNode> for BTNode {
    fn from(node: WaitDurationActionNode) -> Self {
        BTNode::Action(ActionBTNode::WaitDuration(node))
    }
}