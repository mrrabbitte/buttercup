use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct ParallelCompositeNode;

impl BehaviorTreeNode for ParallelCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}
