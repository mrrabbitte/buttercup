use serde::{Deserialize, Serialize};

use buttercup_bts::node::{BehaviorTreeNode, BTNode};
use buttercup_bts::node::decorator::DecoratorBTNode;
use buttercup_bts::node::decorator::reactive::ReactiveConditionDecoratorNode;
use buttercup_bts::node::root::one_off::OneOffRootBTNode;
use buttercup_bts::node::root::reactive::ReactiveRootBTNode;
use buttercup_bts::node::root::RootBTNode;
use buttercup_bts::node::root::to_first::ToFirstErrorRootBTNode;
use buttercup_bts::node::root::until_stopped::UntilStoppedRootBTNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub enum RootNodeDefinition {

    OneOffRootBTNodeDefinition(OneOffRootBTNodeDefinition),
    ReactiveRootBTNodeDefinition(ReactiveRootBTNodeDefinition),
    ToFirstErrorRootBTNodeDefinition(ToFirstErrorRootBTNodeDefinition),
    UntilStoppedRootBTNodeDefinition(UntilStoppedRootBTNodeDefinition)

}

impl RootBTNodeDefinition for RootNodeDefinition {
    fn build(&self,
                        context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        match self {
            RootNodeDefinition::OneOffRootBTNodeDefinition(def) =>
                def.build(context),
            RootNodeDefinition::ReactiveRootBTNodeDefinition(def) =>
                def.build(context),
            RootNodeDefinition::ToFirstErrorRootBTNodeDefinition(def) =>
                def.build(context),
            RootNodeDefinition::UntilStoppedRootBTNodeDefinition(def) =>
                def.build(context)
        }
    }
}

pub trait RootBTNodeDefinition {

    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError>;

}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct OneOffRootBTNodeDefinition {

    id: i32,
    child_id: i32

}

impl OneOffRootBTNodeDefinition {

    pub fn new(id: i32,
               child_id: i32) -> OneOffRootBTNodeDefinition {
        OneOffRootBTNodeDefinition {
            id,
            child_id
        }
    }

}

impl RootBTNodeDefinition for OneOffRootBTNodeDefinition {
    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<RootBTNode, BehaviorTreeBuildingError> {
        Result::Ok(
            OneOffRootBTNode::new(self.id, context.build_child(&self.child_id)?).into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
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

impl From<OneOffRootBTNodeDefinition> for RootNodeDefinition {
    fn from(def: OneOffRootBTNodeDefinition) -> Self {
        RootNodeDefinition::OneOffRootBTNodeDefinition(def)
    }
}

impl From<ReactiveRootBTNodeDefinition> for RootNodeDefinition {
    fn from(def: ReactiveRootBTNodeDefinition) -> Self {
        RootNodeDefinition::ReactiveRootBTNodeDefinition(def)
    }
}

impl From<ToFirstErrorRootBTNodeDefinition> for RootNodeDefinition {
    fn from(def: ToFirstErrorRootBTNodeDefinition) -> Self {
        RootNodeDefinition::ToFirstErrorRootBTNodeDefinition(def)
    }
}

impl From<UntilStoppedRootBTNodeDefinition> for RootNodeDefinition {
    fn from(def: UntilStoppedRootBTNodeDefinition) -> Self {
        RootNodeDefinition::UntilStoppedRootBTNodeDefinition(def)
    }
}