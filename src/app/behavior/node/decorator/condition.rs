use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext, BTNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::decorator::DecoratorNode;
use crate::app::conditions::ConditionExpressionWrapper;

pub struct ConditionDecoratorNode {

    condition: ConditionExpressionWrapper

}

impl DecoratorNode for ConditionDecoratorNode {
    fn get_child(&self, context: &BTNodeExecutionContext) -> &BTNode {
        unimplemented!()
    }
}

impl BehaviorTreeNode for ConditionDecoratorNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}