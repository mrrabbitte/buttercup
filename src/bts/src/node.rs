use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::action::ActionBTNode;
use crate::app::behavior::node::composite::CompositeBTNode;
use crate::app::behavior::node::decorator::DecoratorBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use crate::app::blackboards::service::{BlackboardError, BlackboardService};
use crate::app::values::ValuesPayload;

pub mod action;
pub mod composite;
pub mod decorator;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum BTNode {

    Action(ActionBTNode),
    Composite(CompositeBTNode),
    Decorator(DecoratorBTNode)

}

#[async_trait(?Send)]
pub trait BehaviorTreeNode {

    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError>;

}

#[async_trait(?Send)]
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