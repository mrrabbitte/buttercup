use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
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
            self.child.tick(header, context).await;
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}