use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError};
use buttercup_values::ValuesPayload;

use crate::context::BTNodeExecutionContext;
use crate::node::action::ActionBTNode;
use crate::node::composite::CompositeBTNode;
use crate::node::decorator::DecoratorBTNode;
use crate::node::decorator::reactive::{DataChangeHandlingError, DataChangeHandlingStatus};
use crate::tick::{TickError, TickStatus};

pub mod action;
pub mod composite;
pub mod decorator;
pub mod root;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum BTNode {

    Action(ActionBTNode),
    Composite(CompositeBTNode),
    Decorator(DecoratorBTNode)

}

#[async_trait]
pub trait BehaviorTreeNode {

    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError>;

}

#[async_trait]
impl BehaviorTreeNode for BTNode {

    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            BTNode::Action(node) => node.tick(context).await,
            BTNode::Composite(node) => node.tick(context).await,
            BTNode::Decorator(node) => node.tick(context).await,
        }
    }

}

impl From<ActionBTNode> for BTNode {
    fn from(node: ActionBTNode) -> Self {
        BTNode::Action(node)
    }
}

impl From<CompositeBTNode> for BTNode {
    fn from(node: CompositeBTNode) -> Self {
        BTNode::Composite(node)
    }
}

impl From<DecoratorBTNode> for BTNode {
    fn from(node: DecoratorBTNode) -> Self {
        BTNode::Decorator(node)
    }
}
