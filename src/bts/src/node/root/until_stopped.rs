use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::context::BTNodeExecutionContext;
use crate::definitions::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::root::{RootBTNode, RootBTNodeDefinition};
use crate::tick::{TickError, TickHeader, TickStatus};

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

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let new_header = header.with_new_root_tick_id(Uuid::new_v4());

            self.child.tick(&new_header, context).await;
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

pub struct UntilStoppedRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl RootBTNodeDefinition for UntilStoppedRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            UntilStoppedRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}