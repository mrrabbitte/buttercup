use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct FallbackCompositeNode {

    children: Vec<BTNode>

}

impl BehaviorTreeNode for FallbackCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &self.children {
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => return Result::Ok(TickStatus::Success),
                    TickStatus::Failure => {},
                    TickStatus::Running(addr) =>
                        return Result::Ok(TickStatus::Running(addr)),
                },
                Err(err) => {},
            }
        }
        return Result::Ok(TickStatus::Failure);
    }
}