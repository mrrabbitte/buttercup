use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeBuildingService, BehaviorTreeDefinition, BehaviorTreeDefinitionService};
use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;
use buttercup_bts::tree::BehaviorTreeService;

#[test]
fn test_only_one_action_node() {
    let tree_definition = BehaviorTreeDefinition::new(1,
                                vec![
                                    Arc::new(PrintLogActionNodeDefinition::new(
                                        1, "Hello!".to_owned()))],
                                Box::new(
                                    OneOffRootBTNodeDefinition::new(2, 1))
    );

    check_builds_ok(tree_definition);
}

#[test]
fn test_fails_when_child_node_definition_is_missing() {
    let tree_definition = BehaviorTreeDefinition::new(1,
                                                      vec![],
                                                      Box::new(
                                                          OneOffRootBTNodeDefinition::new(2, 1))
    );

    check_build_fails(tree_definition);
}

fn check_builds_ok(definition: BehaviorTreeDefinition) {
    build_and_check(definition, true);
}

fn check_build_fails(definition: BehaviorTreeDefinition) {
    build_and_check(definition, false);
}

fn build_and_check(definition: BehaviorTreeDefinition,
                   expected_result: bool) {
    let definition_service = BehaviorTreeDefinitionService::default();

    let definition_id = *definition.get_id();
    definition_service.insert(definition);

    let bt_building_service =
        BehaviorTreeBuildingService::new(
            Arc::new(BehaviorTreeService::default()),
            Arc::new(definition_service));

    let result =
        bt_building_service.build(&definition_id);

    assert_eq!(expected_result, result.is_ok());
}