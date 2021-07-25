use buttercup_bts::node::BTNode;
use buttercup_bts::node::composite::fallback::FallbackCompositeNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

pub struct FallbackCompositeNodeDefinition {

    id: i32,
    children_ids: Vec<i32>

}

impl FallbackCompositeNodeDefinition {

    pub fn new(id: i32,
               children_ids: Vec<i32>) -> FallbackCompositeNodeDefinition {
        FallbackCompositeNodeDefinition {
            id,
            children_ids
        }
    }

}

impl BehaviorTreeNodeDefinition for FallbackCompositeNodeDefinition {
    fn build(&self, context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Ok(
            FallbackCompositeNode::new(
                self.id,
                context.build_children(&self.children_ids)?)
                .into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}