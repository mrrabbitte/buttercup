use crate::bts::{BehaviorTreeNodeDefinition, BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use buttercup_bts::node::BTNode;
use buttercup_bts::node::action::wait::WaitDurationActionNode;

pub struct WaitDurationActionNodeDefinition {

    id: i32


}

impl BehaviorTreeNodeDefinition for WaitDurationActionNodeDefinition {
    fn build(&self,
             _: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Result::Ok(WaitDurationActionNode::new(self.id,))
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}