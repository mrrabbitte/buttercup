use crate::bts::{BehaviorTreeNodeDefinition, BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use buttercup_bts::node::BTNode;
use buttercup_bts::node::decorator::invert::InvertDecoratorNode;

pub struct InvertDecoratorNodeDefinition {

    id: i32,
    child_id: i32

}

impl BehaviorTreeNodeDefinition for InvertDecoratorNodeDefinition {
    fn build(&self,
             ctx: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            InvertDecoratorNode::new(
                self.id,
                ctx.build_child(&self.child_id)?.into())
                .into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}