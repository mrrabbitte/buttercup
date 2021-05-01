use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::definitions::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::root::{RootBTNode, RootBTNodeDefinition};
use crate::tick::{TickError, TickHeader, TickStatus};

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

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        self.child.tick(header, context).await
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

pub struct OneOffRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl RootBTNodeDefinition for OneOffRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            OneOffRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}