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

    Composite(CompositeBTNode),
    Decorator(DecoratorBTNode),
    Action(ActionBTNode)

}

pub struct BTNodeExecutionContext;

pub trait BehaviorTreeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError>;

}

impl BehaviorTreeNode for BTNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            BTNode::Composite(node) => node.tick(context),
            BTNode::Decorator(node) => node.tick(context),
            BTNode::Action(node) => node.tick(context),
        }
    }

}


