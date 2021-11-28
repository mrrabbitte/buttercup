use serde::{Deserialize, Serialize};

use buttercup_bts::node::BTNode;
use buttercup_bts::node::composite::sequence::SequenceCompositeNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct SequenceCompositeNodeDefinition {

    id: i32,
    children_ids: Vec<i32>

}

impl SequenceCompositeNodeDefinition {

    pub fn new(id: i32,
               children_ids: Vec<i32>) -> SequenceCompositeNodeDefinition {
        SequenceCompositeNodeDefinition {
            id,
            children_ids
        }
    }

}

impl BehaviorTreeNodeDefinition for SequenceCompositeNodeDefinition {
    fn build(&self, context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Ok(
            SequenceCompositeNode::new(
                self.id,
                context.build_children(&self.children_ids)?)
                .into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}