use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

pub struct ConditionDecoratorNode;

impl BehaviorTreeNode for ConditionDecoratorNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}