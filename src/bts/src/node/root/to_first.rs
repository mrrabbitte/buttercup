use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

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
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let result = self.child.tick(context).await;
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
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            if let Err(err) = self.child.tick(context).await {
                return Result::Err(err);
            }
        }
    }
}