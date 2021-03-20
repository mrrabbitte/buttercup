use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::logging::PrintLogActionNode;
use crate::node::action::subtree::ExecuteSubTreeActionNode;
use crate::node::action::wait::WaitDurationActionNode;
use crate::tick::{TickError, TickHeader, TickStatus};

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

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            ActionBTNode::ExecuteSubTree(node) =>
                node.do_tick(header, context).await,
            ActionBTNode::PrintLog(node) =>
                node.do_tick(header, context).await,
            ActionBTNode::WaitDuration(node) =>
                node.do_tick(header, context).await,
        }
    }

    fn get_id(&self) -> &i32 {
        match self {
            ActionBTNode::ExecuteSubTree(node) => node.get_id(),
            ActionBTNode::PrintLog(node) => node.get_id(),
            ActionBTNode::WaitDuration(node) => node.get_id(),
        }
    }
}

