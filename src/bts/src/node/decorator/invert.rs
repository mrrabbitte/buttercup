use std::sync::Arc;

use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::node::decorator::DecoratorBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct InvertDecoratorNode {

    id: i32,
    child: Box<BTNode>

}

#[async_trait(?Send)]
impl BehaviorTreeNode for InvertDecoratorNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.child.tick(context).await {
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