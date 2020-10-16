use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};
use std::sync::Arc;

pub struct InvertDecoratorNode {

    address: BTNodeAddress,
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