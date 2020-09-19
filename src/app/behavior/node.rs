use serde::{Deserialize, Serialize};

use buttercup_macros::Address;

use crate::app::address::Address;
use crate::app::behavior::node::action::ActionBTNode;
use crate::app::behavior::node::composite::CompositeBTNode;
use crate::app::behavior::node::decorator::DecoratorBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};

mod action;
mod decorator;
mod composite;

#[derive(Address, Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct BTNodeAddress {

    id: i32,
    index: usize

}

pub enum BTNode {

    Action(ActionBTNode),
    Composite(CompositeBTNode),
    Decorator(DecoratorBTNode)

}

pub struct BTNodeExecutionContext;

impl BTNodeExecutionContext {

    pub fn new() -> BTNodeExecutionContext {
        BTNodeExecutionContext {}
    }

}

pub trait BehaviorTreeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError>;

}

impl BehaviorTreeNode for BTNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            BTNode::Action(node) => node.tick(context),
            BTNode::Composite(node) => node.tick(context),
            BTNode::Decorator(node) => node.tick(context),
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
