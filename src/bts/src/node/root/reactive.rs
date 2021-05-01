use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::context::BTNodeExecutionContext;
use crate::definitions::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::DecoratorBTNode;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::node::root::{RootBTNode, RootBTNodeDefinition};
use crate::tick::{TickError, TickHeader, TickStatus};

pub struct ReactiveRootBTNode {

    id: i32,
    child: Box<ReactiveConditionDecoratorNode>,
    stop_on_error: bool

}

#[async_trait]
impl BehaviorTreeNode for ReactiveRootBTNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        loop {
            let new_header = header.with_new_root_tick_id(Uuid::new_v4());

            let result = self.child.tick(&new_header, context).await;

            if let Ok(TickStatus::Success) = result {
                continue;
            } else {
                return result;
            }
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl ReactiveRootBTNode {

    pub fn new(id: i32,
               child: Box<ReactiveConditionDecoratorNode>,
               stop_on_error: bool) -> ReactiveRootBTNode {
        ReactiveRootBTNode {
            id,
            child,
            stop_on_error
        }
    }
}

pub struct ReactiveRootBTNodeDefinition {

    id: i32,
    child_id: i32,
    stop_on_error: bool

}

impl ReactiveRootBTNodeDefinition {

    fn get_reactive_node(bt_node: BTNode)
        -> Result<ReactiveConditionDecoratorNode, BehaviorTreeBuildingError> {
        let node_id = bt_node.get_id();

        match bt_node {
            BTNode::Decorator(
                DecoratorBTNode::ReactiveCondition(node)) =>
                Result::Ok(node),

            _ => Result::Err(BehaviorTreeBuildingError::GotUnexpectedNodeType(*node_id))
        }
    }

}

impl RootBTNodeDefinition for ReactiveRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            ReactiveRootBTNode::new(
                self.id,
                Box::new(
                    ReactiveRootBTNodeDefinition::get_reactive_node(
                        context.build_child(&self.child_id)?)?),
                self.stop_on_error).into())
    }
}