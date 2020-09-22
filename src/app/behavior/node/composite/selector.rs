use crate::app::behavior::node::{BehaviorTreeNode, BTNode, BTNodeExecutionContext};
use crate::app::behavior::node::composite::CompositeNode;
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct SelectorCompositeNode {

    children: Vec<BTNode>

}

impl BehaviorTreeNode for SelectorCompositeNode {
    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        for child in self.get_children() {
            match child.tick(context) {
                Ok(status) => match status {
                    TickStatus::Success => return Result::Ok(TickStatus::Success),
                    TickStatus::Failure => {},
                    TickStatus::Running => return Result::Ok(TickStatus::Running),
                },
                Err(err) => {},
            }
        }
        return Result::Ok(TickStatus::Failure);
    }
}

impl CompositeNode for SelectorCompositeNode {
    fn get_children(&self) -> &Vec<BTNode> {
        unimplemented!()
    }
}