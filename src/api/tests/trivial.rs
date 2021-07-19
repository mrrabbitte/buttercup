use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeBuildingService, BehaviorTreeDefinition, BehaviorTreeDefinitionService};
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;
use buttercup_bts::tree::BehaviorTreeService;

#[test]
fn test_only_one_action_node() {
    let tree_definition = BehaviorTreeDefinition::new(1,
                                vec![],
                                Box::new(
                                    OneOffRootBTNodeDefinition::new(2, 1))
    );

    check_builds_ok(tree_definition);
}


fn check_builds_ok(definition: BehaviorTreeDefinition) {
    let definition_service = BehaviorTreeDefinitionService::default();

    let definition_id = *definition.get_id();
    definition_service.insert(definition);

    let bt_building_service =
        BehaviorTreeBuildingService::new(
            Arc::new(BehaviorTreeService::default()),
            Arc::new(definition_service));

    let result =
        bt_building_service.build(&definition_id);

    assert_eq!(true, result.is_ok());
}