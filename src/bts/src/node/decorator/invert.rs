use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::DecoratorBTNode;
use crate::tick::{TickError, TickStatus, TickHeader};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct InvertDecoratorNode {

    id: i32,
    child: Box<BTNode>

}

impl InvertDecoratorNode {

    pub fn new(id: i32, child: Box<BTNode>) -> InvertDecoratorNode {
        InvertDecoratorNode {
            id,
            child
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for InvertDecoratorNode {
    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.child.as_ref().tick(header, context).await {
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

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<InvertDecoratorNode> for BTNode {
    fn from(node: InvertDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::Invert(node))
    }
}