use std::sync::Arc;

use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::{BehaviorTreeDefinition, BehaviorTreeBuildingError};
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;

mod common;

#[test]
fn test_only_one_action_node() {
    let tree_definition = BehaviorTreeDefinition::new(1,
                                vec![
                                    Arc::new(PrintLogActionNodeDefinition::new(
                                        1, "Hello!".to_owned()).into())],
                                Box::new(
                                    OneOffRootBTNodeDefinition::new(2, 1).into())
    );

    common::check_builds_ok(tree_definition);
}

#[test]
fn test_fails_when_child_node_definition_is_missing() {
    let tree_definition = BehaviorTreeDefinition::new(1,
                                                      vec![],
                                                      Box::new(
                                                          OneOffRootBTNodeDefinition::new(
                                                              2, 1).into())
    );

    common::check_build_fails(tree_definition,
                              BehaviorTreeBuildingError::CouldNotFindChildDefinitionWithId(1));
}