use std::sync::Arc;

use async_trait::async_trait;

use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::node::composite::fallback::FallbackCompositeNode;
use crate::app::behavior::node::composite::parallel::ParallelCompositeNode;
use crate::app::behavior::node::composite::sequence::SequenceCompositeNode;
use crate::app::behavior::tick::{TickError, TickStatus};

mod parallel;
mod fallback;
mod sequence;

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
