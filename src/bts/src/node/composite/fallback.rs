use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::CompositeBTNode;
use crate::tick::{TickError, TickStatus, TickHeader};
use std::sync::Arc;

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

#[async_trait]
impl BehaviorTreeNode for FallbackCompositeNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        let mut errs = Vec::new();

        for child in &self.children {
            match child.tick(header, context).await {
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

        return Result::Err(TickError::CompositeError(self.id, Arc::new(errs)));
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<FallbackCompositeNode> for BTNode {
    fn from(node: FallbackCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Fallback(node))
    }
}