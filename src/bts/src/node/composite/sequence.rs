use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::CompositeBTNode;
use crate::tick::{TickError, TickStatus, TickHeader};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SequenceCompositeNode {

    id: i32,
    children: Vec<BTNode>,

}

impl SequenceCompositeNode {

    pub fn new(id: i32,
               children: Vec<BTNode>) -> SequenceCompositeNode {
        SequenceCompositeNode {
            id,
            children
        }
    }

}

#[async_trait]
impl BehaviorTreeNode for SequenceCompositeNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {

        for child in &self.children {
            match child.tick(header, context).await {
                Ok(status) => match status {
                    TickStatus::Success => {},
                    TickStatus::Failure => {
                        return Result::Ok(TickStatus::Failure);
                    },
                },
                Err(err) => {
                    return Result::Err(err);
                },
            }
        }

        Result::Ok(TickStatus::Success)
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl From<SequenceCompositeNode> for BTNode {
    fn from(node: SequenceCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Sequence(node))
    }
}