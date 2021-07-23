use std::ops::Deref;
use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeDefinition, BehaviorTreeNodeDefinition};
use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::composite::fallback::FallbackCompositeNodeDefinition;
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;

mod common;

#[test]
fn test_builds_fallback_node_correctly() {
    let (children, fallback_node_id) = fallback_node(
        vec![
            Arc::new(
                PrintLogActionNodeDefinition::new(
                    1, "Hello!".to_owned()))]
    );

    let tree_definition =
        common::one_off_root_tree(fallback_node_id,
                                  children);

    common::check_builds_ok(tree_definition);
}

#[test]
fn test_builds_multiple_fallback_nodes_correctly() {
    let (children, fallback_node_id) =
        add_fallback_node(
            vec![
                fallback_node_with_print_log_actions(vec![1, 2, 3, 4]),
                add_fallback_node(
                    vec![
                        fallback_node_with_print_log_actions(vec![5, 6]),
                        fallback_node_with_print_log_actions(vec![7]),
                        fallback_node_with_print_log_actions(vec![8, 9, 10])]),
                fallback_node_with_print_log_actions(vec![11, 12])
            ]
        );

    let tree_definition =
        common::one_off_root_tree(fallback_node_id,
                                  children);

    common::check_builds_ok(tree_definition);
}

fn add_fallback_node(responses: Vec<(Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)>)
    -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    let mut children: Vec<Arc<dyn BehaviorTreeNodeDefinition>> =
        responses
            .into_iter()
            .flat_map(|entry| entry.0)
            .collect();

    fallback_node(children)
}

fn fallback_node_with_print_log_actions(ids: Vec<i32>)
    -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    let mut nodes: Vec<Arc<dyn BehaviorTreeNodeDefinition>> = Vec::new();

    for id in ids {
        nodes.push(Arc::new(
            PrintLogActionNodeDefinition::new(
                id, "Hello!".to_owned())));
    }

    fallback_node(nodes)
}

fn fallback_node(children: Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
    -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    let ids: Vec<i32> = children
        .iter()
        .map(Deref::deref)
        .map(BehaviorTreeNodeDefinition::get_id)
        .map(Clone::clone)
        .collect();

    let fallback_id = *ids.iter().max().expect("Got empty child ids vec.") + 1;

    let mut response = Vec::new();

    response.extend(children);

    response.push(
        Arc::new(FallbackCompositeNodeDefinition::new(
            fallback_id,ids)
        )
    );

    (response, fallback_id)
}