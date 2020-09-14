use crate::app::behavior::node::action::logging::PrintLogActionNode;
use crate::app::behavior::node::{BehaviorTreeNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickStatus, TickError};

mod logging;

pub enum ActionBTNode {

    PrintLog(PrintLogActionNode)

}

impl BehaviorTreeNode for ActionBTNode {

    fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match &self {
            ActionBTNode::PrintLog(node) => node.tick(context)
        }
    }

}