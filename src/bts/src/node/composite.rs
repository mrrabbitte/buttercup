use std::sync::Arc;

use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::fallback::FallbackCompositeNode;
use crate::node::composite::parallel::ParallelCompositeNode;
use crate::node::composite::sequence::SequenceCompositeNode;
use crate::tick::{TickError, TickHeader, TickStatus};

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

#[async_trait]
impl BehaviorTreeNode for CompositeBTNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            CompositeBTNode::Parallel(node) =>
                node.do_tick(header, context).await,
            CompositeBTNode::Fallback(node) =>
                node.do_tick(header, context).await,
            CompositeBTNode::Sequence(node) =>
                node.do_tick(header, context).await,
        }
    }

    fn get_id(&self) -> &i32 {
        match self {
            CompositeBTNode::Parallel(node) => node.get_id(),
            CompositeBTNode::Fallback(node) => node.get_id(),
            CompositeBTNode::Sequence(node) => node.get_id(),
        }
    }
}
