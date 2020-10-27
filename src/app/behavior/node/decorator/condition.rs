use std::collections::HashSet;
use std::future::Future;
use std::iter::FromIterator;
use std::ops::Deref;
use std::sync::Arc;

use actix_web::guard::Guard;
use async_trait::async_trait;
use futures::future::{Abortable, Aborted, AbortHandle};

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::context::reactive::ReactiveServiceError;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::node::decorator::DecoratorBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use crate::app::blackboards::service::{BlackboardError, BlackboardService};
use crate::app::conditions::ConditionExpressionWrapper;
use crate::app::values::ValuesPayload;

pub struct ConditionDecoratorNode {

    id: i32,
    child: Box<BTNode>,
    predicate: Box<dyn Fn(&ValuesPayload)  -> bool + Send + Sync>,
    value_names: HashSet<String>

}

impl ConditionDecoratorNode {

    pub fn new(id: i32,
               child: Box<BTNode>,
               condition: ConditionExpressionWrapper) -> ConditionDecoratorNode {
        let value_names = condition.get_value_names_cloned();
        ConditionDecoratorNode {
            id,
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
            Err(err) => Result::Err(TickError::BlackboardError(self.id, err))
        }
    }
}

impl From<ConditionDecoratorNode> for BTNode {
    fn from(node: ConditionDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::Condition(node))
    }
}

