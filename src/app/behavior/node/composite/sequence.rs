use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext, BTNode};
use crate::app::behavior::tick::{TickStatus, TickError};
use crate::app::behavior::node::composite::CompositeNode;

pub struct SequenceCompositeNode;

impl BehaviorTreeNode for SequenceCompositeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in self.get_children() {
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => {},
                    TickStatus::Failure => return Result::Ok(TickStatus::Failure),
                    TickStatus::Running => return Result::Ok(TickStatus::Running),
                },
                Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(TickStatus::Success)
    }

}

impl CompositeNode for SequenceCompositeNode {
    fn get_children(&self, context: &BTNodeExecutionContext) -> &Vec<BTNode> {
        unimplemented!()
    }
}