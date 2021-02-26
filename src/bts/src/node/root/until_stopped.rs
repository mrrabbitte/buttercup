use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

pub struct UntilStoppedRootBTNode {

    id: i32,
    child: Arc<BTNode>

}

#[async_trait(?Send)]
impl BehaviorTreeNode for UntilStoppedRootBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            self.child.tick(&context).await;
        }
    }
}