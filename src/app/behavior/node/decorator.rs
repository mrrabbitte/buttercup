use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::decorator::condition::ConditionDecoratorNode;
use crate::app::behavior::node::decorator::invert::InvertDecoratorNode;

mod condition;
mod invert;

pub enum DecoratorBTNode {

    Condition(ConditionDecoratorNode),
    Invert(InvertDecoratorNode)

}

impl BehaviorTreeNode for DecoratorBTNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            DecoratorBTNode::Condition(node) => node.tick(context),
            DecoratorBTNode::Invert(node) => node.tick(context),
        }
    }
}