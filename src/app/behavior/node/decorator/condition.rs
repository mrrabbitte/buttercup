use async_trait::async_trait;

use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::conditions::ConditionExpressionWrapper;
use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::values::ValuesPayload;
use std::collections::HashSet;
use crate::app::blackboards::service::{BlackboardService, BlackboardError};
use actix_web::guard::Guard;
use std::ops::Deref;
use std::iter::FromIterator;
use std::future::Future;
use std::sync::Arc;
use crate::app::behavior::node::decorator::DecoratorBTNode;

pub struct ConditionDecoratorNode {

    child: Box<BTNode>,
    predicate: Box<dyn Fn(&ValuesPayload) -> bool>,
    value_names: HashSet<String>

}

impl ConditionDecoratorNode {

    pub fn new(child: Box<BTNode>,
               condition: ConditionExpressionWrapper) -> ConditionDecoratorNode {
        let value_names = condition.get_value_names_cloned();
        ConditionDecoratorNode {
            child,
            predicate: condition.unpack(),
            value_names
        }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ConditionDecoratorNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match context.get_values(&self.value_names) {
            Ok(payload) => {
                if self.predicate.deref()(&payload) {
                    return self.child.tick(context).await;
                }
                return Result::Ok(TickStatus::Failure);
            }
            Err(err) => Result::Err(TickError::BlackboardError(err))
        }
    }
}

impl From<ConditionDecoratorNode> for BTNode {
    fn from(node: ConditionDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::Condition(node))
    }
}