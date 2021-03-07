use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

pub struct ToFirstFailRootBTNode {

    id: i32,
    child: Arc<BTNode>,
    stop_on_failed_status: bool

}

#[async_trait]
impl BehaviorTreeNode for ToFirstFailRootBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let result = self.child.tick(context).await;
            if let Err(err) = result {
                return Result::Err(err);
            }
            if let Ok(TickStatus::Failure) = result {
                if self.stop_on_failed_status {
                    return Result::Ok(TickStatus::Failure);
                }
            }
        }
    }
}