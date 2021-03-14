use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

pub struct OneOffRootBTNode {

    id: i32,
    child: Box<BTNode>

}

impl OneOffRootBTNode {

    pub fn new(id: i32,
               child: BTNode) -> OneOffRootBTNode {
        OneOffRootBTNode {
            id,
            child: Box::new(child)
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for OneOffRootBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        self.child.tick(context).await
    }
}