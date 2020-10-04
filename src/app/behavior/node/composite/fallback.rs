use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct FallbackCompositeNode {

    address: BTNodeAddress,
    children: Vec<BTNode>,
    current_idx: usize
}

impl BehaviorTreeNode for FallbackCompositeNode {
    fn tick(&mut self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &mut self.children[self.current_idx..] {
            self.current_idx += 1;
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => {
                        self.current_idx = 0;
                        return Result::Ok(TickStatus::Success);
                    },
                    TickStatus::Failure => {},
                    TickStatus::Running(_) =>
                        return Result::Ok(TickStatus::Running(self.address.clone())),
                },
                Err(err) => {},
            }
        }
        self.current_idx = 0;
        return Result::Ok(TickStatus::Failure);
    }
}