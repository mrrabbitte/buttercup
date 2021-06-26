use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError};
use buttercup_bts::node::root::RootBTNode;
use buttercup_bts::node::root::one_off::OneOffRootBTNode;
use buttercup_bts::node::{BTNode, BehaviorTreeNode};
use buttercup_bts::node::decorator::reactive::ReactiveConditionDecoratorNode;
use buttercup_bts::node::decorator::DecoratorBTNode;
use buttercup_bts::node::root::reactive::ReactiveRootBTNode;
use buttercup_bts::node::root::to_first::ToFirstErrorRootBTNode;
use buttercup_bts::node::root::until_stopped::UntilStoppedRootBTNode;

pub trait RootBTNodeDefinition {

    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError>;

}

pub struct OneOffRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl RootBTNodeDefinition for OneOffRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            OneOffRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}


pub struct ReactiveRootBTNodeDefinition {

    id: i32,
    child_id: i32,
    stop_on_error: bool

}

impl ReactiveRootBTNodeDefinition {

    fn get_reactive_node(bt_node: BTNode)
                         -> Result<ReactiveConditionDecoratorNode, BehaviorTreeBuildingError> {
        let node_id = bt_node.get_id();

        match bt_node {
            BTNode::Decorator(
                DecoratorBTNode::ReactiveCondition(node)) =>
                Result::Ok(node),

            _ => Result::Err(BehaviorTreeBuildingError::GotUnexpectedNodeType(*node_id))
        }
    }

}

impl RootBTNodeDefinition for ReactiveRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            ReactiveRootBTNode::new(
                self.id,
                Box::new(
                    ReactiveRootBTNodeDefinition::get_reactive_node(
                        context.build_child(&self.child_id)?)?),
                self.stop_on_error).into())
    }
}

pub struct ToFirstErrorRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl RootBTNodeDefinition for ToFirstErrorRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            ToFirstErrorRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}

pub struct UntilStoppedRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl RootBTNodeDefinition for UntilStoppedRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            UntilStoppedRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}