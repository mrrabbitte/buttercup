use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeBuildingError, BehaviorTreeBuildingService, BehaviorTreeDefinition, BehaviorTreeDefinitionService, BehaviorTreeNodeDefinition};
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;
use buttercup_bts::tree::{BehaviorTree, BehaviorTreeService};

pub fn check_builds_ok(definition: BehaviorTreeDefinition) {
    build(definition).expect("Expected result to be OK.");
}

pub fn check_build_fails(definition: BehaviorTreeDefinition,
                         expected_error: BehaviorTreeBuildingError) {
    match build(definition) {
        Ok(_) => panic!("Expected Error."),
        Err(err) => assert_eq!(expected_error, err)
    }
}

pub fn one_off_root_tree(child_id: i32,
                         definitions: Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
                         -> BehaviorTreeDefinition {
    one_off_root_tree_with_id(child_id, definitions, 1)
}

pub fn one_off_root_tree_with_id(child_id: i32,
                         definitions: Vec<Arc<dyn BehaviorTreeNodeDefinition>>,
                                 id: i32)
                         -> BehaviorTreeDefinition {
    BehaviorTreeDefinition::new(id,
                                definitions,
                                Box::new(
                                    OneOffRootBTNodeDefinition::new(5436, child_id)
                                )
    )
}

pub fn build_with_subtrees(definition: BehaviorTreeDefinition,
                           subtrees: Vec<BehaviorTreeDefinition>)
                           -> Result<BehaviorTree, BehaviorTreeBuildingError> {
    let definition_service = BehaviorTreeDefinitionService::default();

    let definition_id = *definition.get_id();
    definition_service.insert(definition);

    for subtree in subtrees {
        definition_service.insert(subtree);
    }

    let bt_building_service =
        BehaviorTreeBuildingService::new(
            Arc::new(BehaviorTreeService::default()),
            Arc::new(definition_service));

    let result =
        bt_building_service.build(&definition_id);

    result
}

fn build(definition: BehaviorTreeDefinition) -> Result<BehaviorTree, BehaviorTreeBuildingError> {
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
