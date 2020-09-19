use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext, BTNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::decorator::condition::ConditionDecoratorNode;
use crate::app::behavior::node::decorator::invert::InvertDecoratorNode;

mod condition;
mod invert;

pub enum DecoratorBTNode {

    Condition(ConditionDecoratorNode),
    Invert(InvertDecoratorNode)

}

pub trait DecoratorNode: BehaviorTreeNode {

    fn get_child(&self, context: &BTNodeExecutionContext) -> &BTNode;

}

impl BehaviorTreeNode for DecoratorBTNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            DecoratorBTNode::Condition(node) => node.tick(context),
            DecoratorBTNode::Invert(node) => node.tick(context),
        }
    }
}

impl From<ConditionDecoratorNode> for DecoratorBTNode {
    fn from(node: ConditionDecoratorNode) -> Self {
        DecoratorBTNode::Condition(node)
    }
}

impl From<InvertDecoratorNode> for DecoratorBTNode {
    fn from(node: InvertDecoratorNode) -> Self {
        DecoratorBTNode::Invert(node)
    }
}