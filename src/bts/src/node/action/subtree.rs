use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::action::ActionBTNode;
use crate::tick::{TickError, TickStatus};
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
               parent_tree_id: &i32,
               tree: Arc<BehaviorTree>) -> Result<ExecuteSubTreeActionNode, ExecuteSubTreeActionNodeError> {
        if !tree.can_be_subtree() {
            return Result::Err(ExecuteSubTreeActionNodeError::ProvidedTreeCannotBeASubtree);
        }

        if tree.get_id() == parent_tree_id {
            return Result::Err(ExecuteSubTreeActionNodeError::SubtreeIsParentTree);
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

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum ExecuteSubTreeActionNodeError {

    ProvidedTreeCannotBeASubtree,
    SubtreeIsParentTree,

}

#[async_trait]
impl BehaviorTreeNode for ExecuteSubTreeActionNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        self.tree.tick(context).await
    }
}

impl From<ExecuteSubTreeActionNode> for BTNode {
    fn from(node: ExecuteSubTreeActionNode) -> Self {
        BTNode::Action(ActionBTNode::ExecuteSubTree(node))
    }
}