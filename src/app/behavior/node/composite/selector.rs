use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

pub struct SelectorCompositeNode;

impl BehaviorTreeNode for SelectorCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}