use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::BehaviorTreeNode;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::tick::{TickError, TickStatus};

pub struct ReactiveRootBTNode {

    id: i32,
    child: Arc<ReactiveConditionDecoratorNode>,
    stop_on_error: bool

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ReactiveRootBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let result = self.child.tick(context).await;
            if let Ok(TickStatus::Success) = result {
                continue;
            } else {
                return result;
            }
        }
    }
}