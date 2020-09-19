use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext, BTNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::composite::CompositeNode;

pub struct SelectorCompositeNode;

impl BehaviorTreeNode for SelectorCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }
}

impl CompositeNode for SelectorCompositeNode {
    fn get_children(&self, context: &BTNodeExecutionContext) -> &Vec<BTNode> {
        unimplemented!()
    }
}