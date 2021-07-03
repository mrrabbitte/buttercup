use buttercup_bts::node::{BehaviorTreeNode, BTNode};
use buttercup_bts::node::decorator::condition::ConditionDecoratorNode;
use buttercup_conditions::{ConditionExpression, ConditionExpressionWrapper};

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

pub struct ConditionDecoratorNodeDefinition {

    id: i32,
    child_id: i32,
    expression: ConditionExpression

}

impl BehaviorTreeNodeDefinition for ConditionDecoratorNodeDefinition {
    fn build(&self, context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Ok(
            ConditionDecoratorNode::new(
                self.id,
                context.build_child(&self.child_id)?,
                ConditionExpressionWrapper::new(self.expression.clone())
            ).into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id 
    }
}