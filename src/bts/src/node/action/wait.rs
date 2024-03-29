use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use async_std::task;
use async_trait::async_trait;

use buttercup_blackboards::LocalBlackboardError;
use buttercup_values::ValueHolder;
use buttercup_variables::{VariableSpecification, VariableValueAccessError};

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::ActionBTNode;
use crate::tick::{TickError, TickHeader, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WaitDurationActionNode {

    id: i32,

    #[derivative(Debug(format_with="WaitDurationActionNode::fmt"))]
    duration: VariableSpecification<Duration>

}

impl WaitDurationActionNode {

    pub fn new(id: i32,
               duration: VariableSpecification<Duration>) -> WaitDurationActionNode {
        WaitDurationActionNode {
            id,
            duration
        }
    }

    fn fmt(spec: &VariableSpecification<Duration>,
           formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match spec {
            VariableSpecification::Literal(duration) =>
                formatter.write_str(format!("{} ms", duration.as_millis()).as_str())?,
            VariableSpecification::VariableName(name) =>
                formatter.write_str(name.get_value())?
        }
        Result::Ok(())
    }

}

#[async_trait]
impl BehaviorTreeNode for WaitDurationActionNode {

    async fn do_tick(&self,
                     _: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.duration.get_value(context) {
            Ok(duration) => {
                task::sleep(duration.deref().clone()).await;
                Result::Ok(TickStatus::Success)
            }
            Err(err) =>
                Result::Err(TickError::VariableValueAccessError(self.id, err))
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<WaitDurationActionNode> for BTNode {
    fn from(node: WaitDurationActionNode) -> Self {
        BTNode::Action(ActionBTNode::WaitDuration(node))
    }
}
