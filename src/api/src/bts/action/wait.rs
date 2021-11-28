use std::time::Duration;

use serde::{Deserialize, Serialize};

use buttercup_bts::node::action::wait::WaitDurationActionNode;
use buttercup_bts::node::BTNode;
use buttercup_variables::VariableSpecification;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct WaitDurationActionNodeDefinition {

    id: i32,
    duration: VariableSpecification<Duration>

}

impl BehaviorTreeNodeDefinition for WaitDurationActionNodeDefinition {
    fn build(&self,
             _: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            WaitDurationActionNode::new(self.id, self.duration.clone())
                .into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}
