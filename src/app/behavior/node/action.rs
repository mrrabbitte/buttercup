use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNodeAddress};
use crate::app::behavior::node::action::logging::PrintLogActionNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use std::sync::Arc;

mod logging;

pub enum ActionBTNode {

    PrintLog(PrintLogActionNode)

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ActionBTNode {

    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            ActionBTNode::PrintLog(node) => node.tick(context).await
        }
    }

}