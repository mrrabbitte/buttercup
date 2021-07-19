use buttercup_bts::node::BTNode;
use buttercup_bts::node::composite::parallel::ParallelCompositeNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

pub struct ParallelCompositeNodeDefinition {

    id: i32,
    children_ids: Vec<i32>,
    num_successes_to_succeed: usize

}

impl BehaviorTreeNodeDefinition for ParallelCompositeNodeDefinition {
    fn build(&self, context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Ok(
            ParallelCompositeNode::new(
                self.id,
                context.build_children(&self.children_ids)?,
                self.num_successes_to_succeed)?
                .into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}