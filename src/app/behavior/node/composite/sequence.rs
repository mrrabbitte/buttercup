use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct SequenceCompositeNode {

    children: Vec<BTNode>

}

impl BehaviorTreeNode for SequenceCompositeNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &self.children {
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => {},
                    TickStatus::Failure => return Result::Ok(TickStatus::Failure),
                    TickStatus::Running(addr) =>
                        return Result::Ok(TickStatus::Running(addr)),
                },
                Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(TickStatus::Success)
    }
}
