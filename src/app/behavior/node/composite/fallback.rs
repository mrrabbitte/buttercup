use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};
use std::sync::Arc;
use crate::app::behavior::node::composite::CompositeBTNode;

pub struct FallbackCompositeNode {

    address: BTNodeAddress,
    children: Vec<BTNode>
}

#[async_trait(?Send)]
impl BehaviorTreeNode for FallbackCompositeNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &self.children {
            match child.tick(context).await {
                Ok(status) => match status {
                    TickStatus::Success => {
                        return Result::Ok(TickStatus::Success);
                    },
                    TickStatus::Failure => {},
                },
                Err(_) => {},
            }
        }
        return Result::Ok(TickStatus::Failure);
    }
}

impl From<FallbackCompositeNode> for BTNode {
    fn from(node: FallbackCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Fallback(node))
    }
}