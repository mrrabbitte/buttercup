use serde::{Deserialize, Serialize};

use buttercup_bts::node::BTNode;
use buttercup_bts::node::decorator::reactive::ReactiveConditionDecoratorNode;
use buttercup_conditions::{ConditionExpression, ConditionExpressionWrapper};

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeNodeDefinition};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct ReactiveConditionDecoratorNodeDefinition {

    id: i32,
    child_id: i32,
    expression: ConditionExpression

}

impl BehaviorTreeNodeDefinition for ReactiveConditionDecoratorNodeDefinition {
    fn build(&self,
             ctx: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        Ok(
            ReactiveConditionDecoratorNode::new(
                self.id,
                ctx.build_child(&self.child_id)?,
                ConditionExpressionWrapper::new(self.expression.clone())
            ).into()
        )
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }
}