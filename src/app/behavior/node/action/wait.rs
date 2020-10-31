use std::convert::TryInto;
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
        WaitDurationActionNode { id, duration: VariableSpecification::Literal(duration) }
    }

}

#[async_trait(?Send)]
impl BehaviorTreeNode for WaitDurationActionNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self.duration {
            VariableSpecification::Literal(duration) =>
                task::sleep(*duration).await,
            VariableSpecification::VariableName(variable_name) =>
                match context.get_value(variable_name) {
                    Ok(Some(value_holder)) => {
                        match value_holder.try_into() {
                            Ok(duration) => task::sleep(*duration).await,
                        }
                    },
                    Ok(None) =>
                        return Result::Err(TickError::VariableValueAccessError(
                            self.id,
                            VariableValueAccessError::VariableOfGivenNameNotFound(
                                variable_name.clone()))),
                    Err(err) =>
                        return Result::Err(TickError::BlackboardError(self.id, err))
                }
        }
        Result::Ok(TickStatus::Success)
    }
}

impl From<WaitDurationActionNode> for BTNode {
    fn from(node: WaitDurationActionNode) -> Self {
        BTNode::Action(ActionBTNode::WaitDuration(node))
    }
}
