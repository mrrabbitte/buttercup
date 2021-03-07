use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::DecoratorBTNode;
use crate::tick::{TickError, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct InvertDecoratorNode {

    id: i32,
    child: Arc<BTNode>

}

#[async_trait]
impl BehaviorTreeNode for InvertDecoratorNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.child.as_ref().tick(context).await {
            Ok(status) =>
                Result::Ok(
                    match status {
                        TickStatus::Success => TickStatus::Failure,
                        TickStatus::Failure => TickStatus::Success
                    }
                ),
            Err(err) =>
                Result::Err(err)
        }
    }
}

impl From<InvertDecoratorNode> for BTNode {
    fn from(node: InvertDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::Invert(node))
    }
}