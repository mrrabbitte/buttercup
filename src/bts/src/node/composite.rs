use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::fallback::FallbackCompositeNode;
use crate::node::composite::parallel::ParallelCompositeNode;
use crate::node::composite::sequence::SequenceCompositeNode;
use crate::tick::{TickError, TickStatus};

pub mod parallel;
pub mod fallback;
pub mod sequence;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum CompositeBTNode {

    Parallel(ParallelCompositeNode),
    Fallback(FallbackCompositeNode),
    Sequence(SequenceCompositeNode)

}

#[async_trait(?Send)]
impl BehaviorTreeNode for CompositeBTNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            CompositeBTNode::Parallel(node) => node.tick(context).await,
            CompositeBTNode::Fallback(node) => node.tick(context).await,
            CompositeBTNode::Sequence(node) => node.tick(context).await,
        }
    }
}
