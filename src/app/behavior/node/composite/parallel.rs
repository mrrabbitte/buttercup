use async_trait::async_trait;

use std::future::Future;

use actix_rt::Arbiter;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct ParallelCompositeNode {

    children: Vec<BTNode>,

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ParallelCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}
