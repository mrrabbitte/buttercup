use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

pub struct UntilStoppedRootBTNode {

    id: i32,
    child: Box<BTNode>

}

impl UntilStoppedRootBTNode {

    pub fn new(id: i32, child: BTNode) -> UntilStoppedRootBTNode {
        UntilStoppedRootBTNode {
            id,
            child: Box::new(child)
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for UntilStoppedRootBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            self.child.tick(&context).await;
        }
    }
}