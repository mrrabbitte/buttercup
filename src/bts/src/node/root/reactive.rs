use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::context::BTNodeExecutionContext;
use crate::node::BehaviorTreeNode;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::tick::{TickError, TickHeader, TickStatus};

pub struct ReactiveRootBTNode {

    id: i32,
    child: Box<ReactiveConditionDecoratorNode>,
    stop_on_error: bool

}

#[async_trait]
impl BehaviorTreeNode for ReactiveRootBTNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let new_header = header.with_new_root_tick_id(Uuid::new_v4());

            let result = self.child.tick(&new_header, context).await;

            if let Ok(TickStatus::Success) = result {
                continue;
            } else {
                return result;
            }
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}