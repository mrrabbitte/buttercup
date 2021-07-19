use std::collections::HashSet;

use buttercup_bts::node::action::subtree::ExecuteSubTreeActionNode;
use buttercup_bts::node::BTNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};

pub struct ExecuteSubTreeActionNodeDefinition {

    id: i32,
    tree_id: i32

}

impl BehaviorTreeNodeDefinition for ExecuteSubTreeActionNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        let subtree = context.get_subtree(&self.tree_id)?;

        Result::Ok(ExecuteSubTreeActionNode::new(self.id, subtree)?.into())
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }

    fn get_subtree_ids(&self,
                       service: &BehaviorTreeDefinitionService)
                       -> Result<HashSet<i32>, BehaviorTreeBuildingError> {
        match service.get(&self.tree_id) {
            None => Result::Err(BehaviorTreeBuildingError::CouldNotFindSubtreeWithId(self.tree_id)),

            Some(subtree) => {
                let mut subtree_ids = subtree.get_subtree_ids(service)?;

                subtree_ids.insert(self.tree_id);

                Result::Ok(subtree_ids)
            }

        }
    }
}

impl From<()> for BehaviorTreeBuildingError {
    fn from(_: ()) -> Self {
        BehaviorTreeBuildingError::ProvidedTreeCannotBeASubtreeError
    }
}
