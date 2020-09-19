use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext, BTNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::composite::CompositeNode;

pub struct SequenceCompositeNode;

impl BehaviorTreeNode for SequenceCompositeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        unimplemented!()
    }

}

impl CompositeNode for BehaviorTreeNode {
    fn get_children(&self, context: &BTNodeExecutionContext) -> &Vec<BTNode> {
        unimplemented!()
    }
}