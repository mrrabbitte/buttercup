use std::collections::HashSet;
use std::future::Future;
use std::iter::FromIterator;
use std::ops::Deref;

use actix_web::guard::Guard;
use async_trait::async_trait;
use futures::future::{Abortable, Aborted, AbortHandle};

use buttercup_blackboards::LocalBlackboardError;
use buttercup_conditions::ConditionExpressionWrapper;
use buttercup_values::ValuesPayload;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::DecoratorBTNode;
use crate::tick::{TickError, TickStatus, TickHeader};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ConditionDecoratorNode {

    id: i32,
    child: Box<BTNode>,

    #[derivative(Debug="ignore")]
    predicate: Box<dyn Fn(&ValuesPayload) -> bool + Send + Sync>,

    value_names: HashSet<String>

}

impl ConditionDecoratorNode {

    pub fn new(id: i32,
               child: BTNode,
               condition: ConditionExpressionWrapper) -> ConditionDecoratorNode {
        let value_names = condition.get_value_names_cloned();
        ConditionDecoratorNode {
            id,
            child: Box::new(child),
            predicate: condition.unpack(),
            value_names
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for ConditionDecoratorNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match context.get_values(&self.value_names) {
            Ok(payload) => {
                if self.predicate.deref()(&payload) {
                    return self.child.tick(header, context).await;
                }
                return Result::Ok(TickStatus::Failure);
            }
            Err(err) => Result::Err(TickError::BlackboardError(self.id, err))
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<ConditionDecoratorNode> for BTNode {
    fn from(node: ConditionDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::Condition(node))
    }
}

