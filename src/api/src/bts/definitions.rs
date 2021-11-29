use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use buttercup_bts::node::BTNode;

use crate::bts::{BehaviorTreeBuildingContext, BehaviorTreeBuildingError, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};
use crate::bts::action::logging::PrintLogActionNodeDefinition;
use crate::bts::action::subtree::ExecuteSubTreeActionNodeDefinition;
use crate::bts::action::wait::WaitDurationActionNodeDefinition;
use crate::bts::composite::fallback::FallbackCompositeNodeDefinition;
use crate::bts::composite::parallel::ParallelCompositeNodeDefinition;
use crate::bts::composite::sequence::SequenceCompositeNodeDefinition;
use crate::bts::decorator::condition::ConditionDecoratorNodeDefinition;
use crate::bts::decorator::invert::InvertDecoratorNodeDefinition;
use crate::bts::decorator::reactive::ReactiveConditionDecoratorNodeDefinition;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub enum BTNodeDefinition {

    ConditionDecoratorNodeDefinition(ConditionDecoratorNodeDefinition),
    ExecuteSubTreeActionNodeDefinition(ExecuteSubTreeActionNodeDefinition),
    FallbackCompositeNodeDefinition(FallbackCompositeNodeDefinition),
    InvertDecoratorNodeDefinition(InvertDecoratorNodeDefinition),
    ParallelCompositeNodeDefinition(ParallelCompositeNodeDefinition),
    PrintLogActionNodeDefinition(PrintLogActionNodeDefinition),
    ReactiveConditionDecoratorNodeDefinition(ReactiveConditionDecoratorNodeDefinition),
    SequenceCompositeNodeDefinition(SequenceCompositeNodeDefinition),
    WaitDurationActionNodeDefinition(WaitDurationActionNodeDefinition)

}

impl BTNodeDefinition {

    fn get_delegate(&self) -> &dyn BehaviorTreeNodeDefinition {
        match self {
            BTNodeDefinition::ConditionDecoratorNodeDefinition(def) => def,
            BTNodeDefinition::ExecuteSubTreeActionNodeDefinition(def) => def,
            BTNodeDefinition::FallbackCompositeNodeDefinition(def) => def,
            BTNodeDefinition::InvertDecoratorNodeDefinition(def) => def,
            BTNodeDefinition::ParallelCompositeNodeDefinition(def) => def,
            BTNodeDefinition::PrintLogActionNodeDefinition(def) => def,
            BTNodeDefinition::ReactiveConditionDecoratorNodeDefinition(def) => def,
            BTNodeDefinition::SequenceCompositeNodeDefinition(def) => def,
            BTNodeDefinition::WaitDurationActionNodeDefinition(def) => def,
        }
    }

}

impl BehaviorTreeNodeDefinition for BTNodeDefinition {
    fn build(&self, ctx: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError> {
        self.get_delegate().build(ctx)
    }

    fn get_id(&self) -> &i32 {
        self.get_delegate().get_id()
    }

    fn get_subtree_ids(&self,
                       service: &BehaviorTreeDefinitionService)
                       -> Result<HashSet<i32>, BehaviorTreeBuildingError> {
        self.get_delegate().get_subtree_ids(service)
    }
}

impl From<ConditionDecoratorNodeDefinition> for BTNodeDefinition {
    fn from(def: ConditionDecoratorNodeDefinition) -> Self {
        BTNodeDefinition::ConditionDecoratorNodeDefinition(def)
    }
}

impl From<ExecuteSubTreeActionNodeDefinition> for BTNodeDefinition {
    fn from(def: ExecuteSubTreeActionNodeDefinition) -> Self {
        BTNodeDefinition::ExecuteSubTreeActionNodeDefinition(def)
    }
}

impl From<FallbackCompositeNodeDefinition> for BTNodeDefinition {
    fn from(def: FallbackCompositeNodeDefinition) -> Self {
        BTNodeDefinition::FallbackCompositeNodeDefinition(def)
    }
}

impl From<ParallelCompositeNodeDefinition> for BTNodeDefinition {
    fn from(def: ParallelCompositeNodeDefinition) -> Self {
        BTNodeDefinition::ParallelCompositeNodeDefinition(def)
    }
}

impl From<PrintLogActionNodeDefinition> for BTNodeDefinition {
    fn from(def: PrintLogActionNodeDefinition) -> Self {
        BTNodeDefinition::PrintLogActionNodeDefinition(def)
    }
}

impl From<ReactiveConditionDecoratorNodeDefinition> for BTNodeDefinition {
    fn from(def: ReactiveConditionDecoratorNodeDefinition) -> Self {
        BTNodeDefinition::ReactiveConditionDecoratorNodeDefinition(def)
    }
}

impl From<SequenceCompositeNodeDefinition> for BTNodeDefinition {
    fn from(def: SequenceCompositeNodeDefinition) -> Self {
        BTNodeDefinition::SequenceCompositeNodeDefinition(def)
    }
}

impl From<WaitDurationActionNodeDefinition> for BTNodeDefinition {
    fn from(def: WaitDurationActionNodeDefinition) -> Self {
        BTNodeDefinition::WaitDurationActionNodeDefinition(def)
    }
}
