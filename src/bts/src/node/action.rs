use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::logging::PrintLogActionNode;
use crate::node::action::subtree::ExecuteSubTreeActionNode;
use crate::node::action::wait::WaitDurationActionNode;
use crate::tick::{TickError, TickStatus};

pub mod logging;
pub mod subtree;
pub mod wait;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ActionBTNode {

    ExecuteSubTree(ExecuteSubTreeActionNode),
    PrintLog(PrintLogActionNode),
    WaitDuration(WaitDurationActionNode)

}

#[async_trait]
impl BehaviorTreeNode for ActionBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            ActionBTNode::ExecuteSubTree(node) => node.tick(context).await,
            ActionBTNode::PrintLog(node) => node.tick(context).await,
            ActionBTNode::WaitDuration(node) => node.tick(context).await,
        }
    }
}

