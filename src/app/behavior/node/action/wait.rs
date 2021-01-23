use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use async_std::task;
use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::node::action::ActionBTNode;
use crate::app::behavior::tick::{TickError, TickStatus};
use crate::app::blackboards::service::BlackboardError;
use crate::app::values::ValueHolder;
use crate::app::variables::{VariableSpecification, VariableValueAccessError};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WaitDurationActionNode {

    id: i32,
    duration: VariableSpecification<Duration>

}

impl WaitDurationActionNode {

    pub fn new(id: i32,
               duration: Duration) -> WaitDurationActionNode {
        WaitDurationActionNode {
            id,
            duration: duration.into()
        }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for WaitDurationActionNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.duration.get_value(context) {
            Ok(duration) => {
                task::sleep(duration.as_ref().clone()).await;
                Result::Ok(TickStatus::Success)
            }
            Err(err) =>
                Result::Err(TickError::VariableValueAccessError(self.id, err))
        }
    }
}

impl From<WaitDurationActionNode> for BTNode {
    fn from(node: WaitDurationActionNode) -> Self {
        BTNode::Action(ActionBTNode::WaitDuration(node))
    }
}
