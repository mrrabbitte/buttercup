use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::CompositeBTNode;
use crate::tick::{TickError, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FallbackCompositeNode {

    id: i32,
    children: Vec<BTNode>
}

impl FallbackCompositeNode {

    pub fn new(id: i32,
               children: Vec<BTNode>) -> FallbackCompositeNode {
        FallbackCompositeNode {
            id,
            children
        }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for FallbackCompositeNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        let mut errs = Vec::new();
        for child in &self.children {
            match child.tick(context).await {
                Ok(status) => match status {
                    TickStatus::Success => {
                        return Result::Ok(TickStatus::Success);
                    },
                    TickStatus::Failure => {},
                },
                Err(err) => errs.push((*err.get_node_id(), err)),
            }
        }
        if errs.is_empty() {
            return Result::Ok(TickStatus::Failure);
        }
        return Result::Err(TickError::CompositeError(self.id, errs));
    }
}

impl From<FallbackCompositeNode> for BTNode {
    fn from(node: FallbackCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Fallback(node))
    }
}