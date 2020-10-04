use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct SequenceCompositeNode {

    address: BTNodeAddress,
    children: Vec<BTNode>,
    current_idx: usize

}

impl BehaviorTreeNode for SequenceCompositeNode {

    fn tick(&mut self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in &mut self.children[self.current_idx..] {
            self.current_idx += 1;
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => {},
                    TickStatus::Failure => {
                        self.current_idx = 0;
                        return Result::Ok(TickStatus::Failure);
                    },
                    TickStatus::Running(addr) =>
                        return Result::Ok(TickStatus::Running(addr)),
                },
                Err(err) => {
                    self.current_idx = 0;
                    return Result::Err(err);
                },
            }
        }
        self.current_idx = 0;
        Result::Ok(TickStatus::Success)
    }
}
