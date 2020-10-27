use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNodeAddress, BTNode};
use crate::app::behavior::node::action::logging::PrintLogActionNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use std::sync::Arc;
use crate::app::behavior::node::action::wait::WaitDurationActionNode;

pub mod logging;
pub mod wait;

pub enum ActionBTNode {

    PrintLog(PrintLogActionNode),
    WaitDuration(WaitDurationActionNode)

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ActionBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            ActionBTNode::PrintLog(node) => node.tick(context).await,
            ActionBTNode::WaitDuration(node) => node.tick(context).await
        }
    }
}

