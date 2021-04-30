use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::context::BTNodeExecutionContext;
use crate::definitions::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeDefinition, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::ActionBTNode;
use crate::tick::{TickError, TickHeader, TickStatus};
use crate::tree::BehaviorTree;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ExecuteSubTreeActionNode {

    id: i32,

    #[derivative(Debug(format_with="ExecuteSubTreeActionNode::fmt"))]
    tree: Arc<BehaviorTree>

}

impl ExecuteSubTreeActionNode {

    pub fn new(id: i32,
               tree: Arc<BehaviorTree>)
        -> Result<ExecuteSubTreeActionNode, ()> {
        if !tree.can_be_subtree() {
            return Result::Err(());
        }

        Result::Ok(
            ExecuteSubTreeActionNode {
                id,
                tree
            }
        )
    }

    fn fmt(tree: &Arc<BehaviorTree>,
           formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str(format!("id: {}", tree.get_id()).as_str());

        Result::Ok(())
    }

}

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

    fn get_subtree_ids(&self,
                       service: &BehaviorTreeDefinitionService)
        -> Result<HashSet<i32>, BehaviorTreeBuildingError> {
        match service.get_definition(&self.tree_id) {
            None => Result::Err(BehaviorTreeBuildingError::CouldNotFindSubtreeWithId(self.tree_id)),

            Some(subtree) => {
                let mut subtree_ids = subtree.get_subtree_ids(service)?;

                subtree_ids.insert(self.tree_id);

                Result::Ok(subtree_ids)
            }

        }
    }
}

#[async_trait]
impl BehaviorTreeNode for ExecuteSubTreeActionNode {
    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        self.tree.subtree_tick(header, context).await
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<ExecuteSubTreeActionNode> for BTNode {
    fn from(node: ExecuteSubTreeActionNode) -> Self {
        BTNode::Action(ActionBTNode::ExecuteSubTree(node))
    }
}

impl From<()> for BehaviorTreeBuildingError {
    fn from(_: ()) -> Self {
        BehaviorTreeBuildingError::ProvidedTreeCannotBeASubtreeError
    }
}