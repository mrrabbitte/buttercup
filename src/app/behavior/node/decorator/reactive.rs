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

pub struct ReactiveConditionDecoratorNode {

    id: i32,
    child: Box<BTNode>,
    predicate: Box<dyn Fn(&ValuesPayload)  -> bool + Send + Sync>,
    value_names: HashSet<String>

}

pub enum DataChangeHandlingStatus {

    AbortedExecution,
    Unchanged

}

pub enum DataChangeHandlingError {

    BlackboardError(i32, BlackboardError),
    ReactiveServiceError(i32, ReactiveServiceError)

}

impl ReactiveConditionDecoratorNode {

    pub fn new(id: i32,
               child: Box<BTNode>,
               condition: ConditionExpressionWrapper) -> ReactiveConditionDecoratorNode {
        let value_names = condition.get_value_names_cloned();
        ReactiveConditionDecoratorNode {
            id,
            child,
            predicate: condition.unpack(),
            value_names
        }
    }

    pub fn handle_data_change_event(&self,
                                    context: &BTNodeExecutionContext)
        -> Result<DataChangeHandlingStatus, DataChangeHandlingError> {
        match context.get_values(&self.value_names) {
            Ok(payload) => {
                if !self.predicate.deref()(&payload) {
                    return match context.get_reactive_service().abort(&self.id) {
                        Ok(_) =>
                            Result::Ok(DataChangeHandlingStatus::AbortedExecution),
                        Err(err) =>
                            Result::Err(DataChangeHandlingError::ReactiveServiceError(self.id, err))
                    };
                }
                return Result::Ok(DataChangeHandlingStatus::Unchanged);
            },
            Err(err) => Result::Err(
                DataChangeHandlingError::BlackboardError(self.id, err))
        }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for ReactiveConditionDecoratorNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match context.get_values(&self.value_names) {
            Ok(payload) => {
                if self.predicate.deref()(&payload) {
                    let (abort_handle, abort_registration) =
                        AbortHandle::new_pair();
                    return
                        match context.get_reactive_service().register(
                            &self.id, abort_handle) {
                            Ok(_) =>
                                match Abortable::new(self.child.tick(context),
                                                     abort_registration).await {
                                    Ok(result) => result,
                                    Err(_) => Result::Ok(TickStatus::Failure)
                                },
                            Err(err) =>
                                Result::Err(TickError::ReactiveServiceError(self.id, err))
                        };
                }
                return Result::Ok(TickStatus::Failure);
            }
            Err(err) => Result::Err(TickError::BlackboardError(self.id, err))
        }
    }
}

impl From<ReactiveConditionDecoratorNode> for BTNode {
    fn from(node: ReactiveConditionDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::ReactiveCondition(node))
    }
}