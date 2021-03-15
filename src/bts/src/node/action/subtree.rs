use std::sync::Arc;

use async_trait::async_trait;

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
               tree: Arc<BehaviorTree>) -> ExecuteSubTreeActionNode {
        ExecuteSubTreeActionNode {
            id,
            tree
        }
    }

    fn fmt(tree: &Arc<BehaviorTree>,
           formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str(format!("{}", tree.get_id()).as_str());

        Result::Ok(())
    }

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