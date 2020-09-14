use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

pub struct ParallelCompositeNode;

impl BehaviorTreeNode for ParallelCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}