use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::node::composite::parallel::ParallelCompositeNode;
use crate::app::behavior::node::composite::selector::SelectorCompositeNode;
use crate::app::behavior::node::composite::sequence::SequenceCompositeNode;
use crate::app::behavior::tick::{TickError, TickStatus};

mod parallel;
mod selector;
mod sequence;

pub enum CompositeBTNode {

    Parallel(ParallelCompositeNode),
    Selector(SelectorCompositeNode),
    Sequence(SequenceCompositeNode)

}

impl BehaviorTreeNode for CompositeBTNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            CompositeBTNode::Parallel(node) => node.tick(context),
            CompositeBTNode::Selector(node) => node.tick(context),
            CompositeBTNode::Sequence(node) => node.tick(context),
        }
    }

}