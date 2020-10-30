use std::sync::Arc;

use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::node::composite::CompositeBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SequenceCompositeNode {

    address: BTNodeAddress,
    children: Vec<BTNode>,

}

#[async_trait(?Send)]
impl BehaviorTreeNode for SequenceCompositeNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &self.children {
            match child.tick(context).await {
                Ok(status) => match status {
                    TickStatus::Success => {},
                    TickStatus::Failure => {
                        return Result::Ok(TickStatus::Failure);
                    },
                },
                Err(err) => {
                    return Result::Err(err);
                },
            }
        }
        Result::Ok(TickStatus::Success)
    }
}

impl From<SequenceCompositeNode> for BTNode {
    fn from(node: SequenceCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Sequence(node))
    }
}