use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeBuildingError, BehaviorTreeBuildingService, BehaviorTreeDefinition, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;
use buttercup_bts::tree::{BehaviorTree, BehaviorTreeService};

pub fn check_builds_ok(definition: BehaviorTreeDefinition) {
    build(definition, true).expect("Got error.");
}

pub fn check_build_fails(definition: BehaviorTreeDefinition,
                         expected_error: BehaviorTreeBuildingError) {
    match build(definition, false) {
        Ok(_) => panic!("Expected error."),
        Err(err) => assert_eq!(expected_error, err)
    }
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

fn build(definition: BehaviorTreeDefinition,
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

    result
}
