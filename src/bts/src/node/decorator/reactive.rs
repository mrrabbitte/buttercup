use std::collections::HashSet;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::{Abortable, Aborted, AbortHandle, AbortRegistration};

use buttercup_blackboards::LocalBlackboardError;
use buttercup_conditions::ConditionExpressionWrapper;
use buttercup_values::ValuesPayload;

use crate::context::BTNodeExecutionContext;
use crate::context::reactive::ReactiveContextError;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::decorator::DecoratorBTNode;
use crate::tick::{TickError, TickStatus, TickHeader};

#[derive(Debug)]
pub struct ReactiveConditionDecoratorNode {

    child: Box<BTNode>,
    inner: Arc<ReactiveConditionInnerNode>

}

pub enum DataChangeHandlingStatus {

    AbortedExecution,
    Unchanged

}

pub enum DataChangeHandlingError {

    BlackboardError(i32, LocalBlackboardError),
    ReactiveServiceError(i32, ReactiveContextError),
    NonReactiveNodeCalledError

}

impl ReactiveConditionDecoratorNode {

    pub fn new(id: i32,
               child: BTNode,
               condition: ConditionExpressionWrapper) -> ReactiveConditionDecoratorNode {
        let value_names = condition.get_value_names_cloned();
        ReactiveConditionDecoratorNode {
            child: Box::new(child),
            inner: Arc::new(ReactiveConditionInnerNode {
                id,
                predicate: condition.unpack(),
                value_names
            })
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for ReactiveConditionDecoratorNode {
    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.inner.register_abortable(&self.inner, context)? {
            None => Result::Ok(TickStatus::Failure),
            Some(abort_registration) =>
                match Abortable::new(self.child.tick(header, context),
                                     abort_registration).await {
                    Ok(result) => result,
                    Err(_) => Result::Ok(TickStatus::Failure)
                },
        }
    }

    fn get_id(&self) -> &i32 {
        self.inner.get_id()
    }
}

impl From<ReactiveConditionDecoratorNode> for BTNode {
    fn from(node: ReactiveConditionDecoratorNode) -> Self {
        BTNode::Decorator(DecoratorBTNode::ReactiveCondition(node))
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ReactiveConditionInnerNode {

    id: i32,

    #[derivative(Debug="ignore")]
    predicate: Box<dyn Fn(&ValuesPayload)  -> bool + Send + Sync>,

    #[derivative(Debug="ignore")]
    value_names: HashSet<String>

}

impl Hash for ReactiveConditionInnerNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for ReactiveConditionInnerNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ReactiveConditionInnerNode {}


impl ReactiveConditionInnerNode {

    pub fn handle_value_change(&self,
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

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_value_names(&self) -> &HashSet<String> {
        &self.value_names
    }

    fn register_abortable(&self,
                          inner: &Arc<ReactiveConditionInnerNode>,
                          context: &BTNodeExecutionContext)
                          -> Result<Option<AbortRegistration>, TickError> {
        match context.get_values(&self.value_names) {
            Ok(payload) => {
                if self.predicate.deref()(&payload) {
                    let (abort_handle, abort_registration) =
                        AbortHandle::new_pair();
                    return
                        match context.get_reactive_service().register(abort_handle, inner) {
                            Ok(_) =>
                                Result::Ok(Option::Some(abort_registration)),
                            Err(err) =>
                                Result::Err(TickError::ReactiveServiceError(self.id, err))
                        };
                }
                return Result::Ok(Option::None);
            }
            Err(err) => Result::Err(TickError::BlackboardError(self.id, err))
        }
    }
}