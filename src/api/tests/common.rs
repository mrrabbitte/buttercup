use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeBuildingError, BehaviorTreeBuildingService, BehaviorTreeDefinition, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};
use buttercup_bts::tree::{BehaviorTree, BehaviorTreeService};
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;

pub fn check_builds_ok(definition: BehaviorTreeDefinition) {
    build_and_check(definition, true);
}

pub fn check_build_fails(definition: BehaviorTreeDefinition) {
    build_and_check(definition, false);
}

pub fn one_off_root_tree(child_id: i32,
                         definitions: Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
                         -> BehaviorTreeDefinition {
    BehaviorTreeDefinition::new(1,
                                definitions,
                                Box::new(
                                    OneOffRootBTNodeDefinition::new(5436, child_id))
    )
}

fn build_and_check(definition: BehaviorTreeDefinition,
                   expected_result: bool) -> Result<BehaviorTree, BehaviorTreeBuildingError> {
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

    result
}
