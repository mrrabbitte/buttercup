use std::sync::Arc;

use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::node::decorator::condition::{ConditionDecoratorNode, ReactiveConditionDecoratorNode};
use crate::app::behavior::node::decorator::invert::InvertDecoratorNode;
use crate::app::behavior::tick::{TickError, TickStatus};

pub(crate) mod condition;
pub(crate) mod invert;

pub enum DecoratorBTNode {

    Condition(ConditionDecoratorNode),
    Invert(InvertDecoratorNode),
    ReactiveCondition(ReactiveConditionDecoratorNode)

}

#[async_trait(?Send)]
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
