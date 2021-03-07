use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::condition::ConditionDecoratorNode;
use crate::node::decorator::invert::InvertDecoratorNode;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::tick::{TickError, TickStatus};

pub mod condition;
pub mod invert;
pub mod reactive;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum DecoratorBTNode {

    Condition(ConditionDecoratorNode),
    Invert(InvertDecoratorNode),
    ReactiveCondition(ReactiveConditionDecoratorNode)

}

#[async_trait]
impl BehaviorTreeNode for DecoratorBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            DecoratorBTNode::Condition(node) =>
                node.tick(context).await,
            DecoratorBTNode::Invert(node) =>
                node.tick(context).await,
            DecoratorBTNode::ReactiveCondition(node) =>
                node.tick(context).await
        }
    }
}
