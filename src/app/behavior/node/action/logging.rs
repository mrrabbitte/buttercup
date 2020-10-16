use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::BehaviorTreeNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use std::sync::Arc;

pub struct PrintLogActionNode;

#[async_trait(?Send)]
impl BehaviorTreeNode for PrintLogActionNode {

    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        println!("I'm here!");
        Result::Ok(TickStatus::Success)
    }

}