use buttercup_bts::node::BTNode;
use buttercup_bts::node::composite::parallel::{ParallelCompositeNode, ParallelCompositeNodeBuildingError};

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

pub struct ParallelCompositeNodeDefinition {

    id: i32,
    children_ids: Vec<i32>,
    num_successes_to_succeed: usize

}

impl ParallelCompositeNodeDefinition {

    pub fn new(id: i32,
               children_ids: Vec<i32>,
               num_successes_to_succeed: usize) -> ParallelCompositeNodeDefinition {
        ParallelCompositeNodeDefinition {
            id,
            children_ids,
            num_successes_to_succeed
        }
    }

}

impl BehaviorTreeNodeDefinition for ParallelCompositeNodeDefinition {
    fn build(&self, context: &BehaviorTreeBuildingContext)
        -> Result<BTNode, BehaviorTreeBuildingError> {
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

impl From<ParallelCompositeNodeBuildingError> for BehaviorTreeBuildingError {
    fn from(_: ParallelCompositeNodeBuildingError) -> Self {
        BehaviorTreeBuildingError::ParallelCompositeNodeBuildingError
    }
}