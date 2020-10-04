use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct InvertDecoratorNode {

    address: BTNodeAddress,
    child: Box<BTNode>

}

impl BehaviorTreeNode for InvertDecoratorNode {

    fn tick(&mut self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self.child.tick(context) {
            Ok(status) =>
                Result::Ok(
                    match status {
                        TickStatus::Success => TickStatus::Failure,
                        TickStatus::Failure => TickStatus::Success,
                        TickStatus::Running(_) => TickStatus::Running(self.address.clone())
                    }
                ),
            Err(err) =>
                Result::Err(err)
        }
    }

}