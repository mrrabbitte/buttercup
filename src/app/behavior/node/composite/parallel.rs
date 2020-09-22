use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeExecutionContext};
use crate::app::behavior::node::composite::CompositeNode;
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct ParallelCompositeNode;

impl BehaviorTreeNode for ParallelCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}

impl CompositeNode for ParallelCompositeNode {

    fn get_children(&self) -> &Vec<BTNode> {
        unimplemented!()
    }
}