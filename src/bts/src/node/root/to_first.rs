use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::root::RootBTNode;
use crate::tick::{TickError, TickHeader, TickStatus};

pub struct ToFirstFailureRootBTNode {

    id: i32,
    child: Box<BTNode>,
    ignore_errors: bool

}

impl ToFirstFailureRootBTNode {

    pub fn new(id: i32,
               child: BTNode,
               ignore_errors: bool) -> ToFirstFailureRootBTNode {
        ToFirstFailureRootBTNode {
            id,
            child: Box::new(child),
            ignore_errors
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for ToFirstFailureRootBTNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let new_header = header.with_new_root_tick_id(Uuid::new_v4());

            let result = self.child.tick(&new_header, context).await;

            if !self.ignore_errors {
                if let Err(_) = result {
                    return result;
                }
            }

            if let Ok(TickStatus::Failure) = result {
                return result;
            }
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

pub struct ToFirstErrorRootBTNode {

    id: i32,
    child: Box<BTNode>

}

impl ToFirstErrorRootBTNode {

    pub fn new(id: i32,
               child: BTNode) -> ToFirstErrorRootBTNode {
        ToFirstErrorRootBTNode {
            id,
            child: Box::new(child)
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for ToFirstErrorRootBTNode {
    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            if let Err(err) = self.child.do_tick(header, context).await {
                return Result::Err(err);
            }
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

