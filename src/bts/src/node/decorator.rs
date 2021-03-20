use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::condition::ConditionDecoratorNode;
use crate::node::decorator::invert::InvertDecoratorNode;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::tick::{TickError, TickHeader, TickStatus};

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

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            DecoratorBTNode::Condition(node) =>
                node.do_tick(header, context).await,
            DecoratorBTNode::Invert(node) =>
                node.do_tick(header, context).await,
            DecoratorBTNode::ReactiveCondition(node) =>
                node.do_tick(header, context).await
        }
    }

    fn get_id(&self) -> &i32 {
        match self {
            DecoratorBTNode::Condition(node) => node.get_id(),
            DecoratorBTNode::Invert(node) => node.get_id(),
            DecoratorBTNode::ReactiveCondition(node) => node.get_id(),
        }
    }
}
