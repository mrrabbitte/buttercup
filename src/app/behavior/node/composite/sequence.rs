use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

pub struct SequenceCompositeNode;

impl BehaviorTreeNode for SequenceCompositeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }

}