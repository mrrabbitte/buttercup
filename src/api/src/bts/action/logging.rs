use serde::{Deserialize, Serialize};

use buttercup_bts::node::action::logging::PrintLogActionNode;
use buttercup_bts::node::BTNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct PrintLogActionNodeDefinition {

    id: i32,
    message: String

}

impl PrintLogActionNodeDefinition {

    pub fn new(id: i32,
               message: String) -> PrintLogActionNodeDefinition {
        PrintLogActionNodeDefinition{
            id,
            message
        }
    }
}

impl BehaviorTreeNodeDefinition for PrintLogActionNodeDefinition {

    fn build(&self,
             _: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Result::Ok(PrintLogActionNode::new(self.id, self.message.clone()).into())
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}