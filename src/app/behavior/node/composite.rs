use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::node::composite::parallel::ParallelCompositeNode;
use crate::app::behavior::node::composite::fallback::FallbackCompositeNode;
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

impl BehaviorTreeNode for CompositeBTNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            CompositeBTNode::Parallel(node) => node.tick(context),
            CompositeBTNode::Fallback(node) => node.tick(context),
            CompositeBTNode::Sequence(node) => node.tick(context),
        }
    }

}

impl From<ParallelCompositeNode> for CompositeBTNode {
    fn from(node: ParallelCompositeNode) -> Self {
        CompositeBTNode::Parallel(node)
    }
}

impl From<FallbackCompositeNode> for CompositeBTNode {
    fn from(node: FallbackCompositeNode) -> Self {
        CompositeBTNode::Fallback(node)
    }
}

impl From<SequenceCompositeNode> for CompositeBTNode {
    fn from(node: SequenceCompositeNode) -> Self {
        CompositeBTNode::Sequence(node)
    }
}
