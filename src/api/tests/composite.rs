use std::ops::Deref;
use std::sync::Arc;

use buttercup_api::bts::{BehaviorTreeDefinition, BehaviorTreeNodeDefinition};
use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::composite::fallback::FallbackCompositeNodeDefinition;
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;
use buttercup_api::bts::composite::sequence::SequenceCompositeNodeDefinition;

mod common;

#[test]
fn test_builds_fallback_node_correctly() {
    let (children, fallback_node_id) =
        fallback_node_with_print_log_actions(vec![1]);

    build_and_check_bt_with_composite(children, fallback_node_id);
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

    build_and_check_bt_with_composite(children, fallback_node_id);
}

#[test]
fn test_builds_sequence_node_correctly() {
    let (children, fallback_node_id) =
        sequence_node_with_print_log_actions(vec![1]);

    build_and_check_bt_with_composite(children, fallback_node_id);
}

#[test]
fn test_builds_multiple_sequence_nodes_correctly() {
    let (children, fallback_node_id) =
        add_sequence_node(
            vec![
                sequence_node_with_print_log_actions(vec![1, 2, 3, 4]),
                add_sequence_node(
                    vec![
                        add_sequence_node(
                            vec![
                                sequence_node_with_print_log_actions(vec![5, 6]),
                                sequence_node_with_print_log_actions(vec![15, 16])
                            ]),
                        sequence_node_with_print_log_actions(vec![7]),
                        sequence_node_with_print_log_actions(vec![8, 9, 10])]),
                sequence_node_with_print_log_actions(vec![11, 12]),
                sequence_node_with_print_log_actions(vec![13, 14])
            ]
        );

    build_and_check_bt_with_composite(children, fallback_node_id);
}

#[test]
fn test_builds_parallel_node_correctly() {

}

#[test]
fn test_builds_multiple_parallel_nodes_correctly() {

}

fn add_composite_node<F>(responses: Vec<(Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)>,
                         composite_node_provider: F)
                         -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)
    where F: Fn(Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
        -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    let mut children: Vec<Arc<dyn BehaviorTreeNodeDefinition>> =
        responses
            .into_iter()
            .flat_map(|entry| entry.0)
            .collect();

    composite_node_provider(children)
}

fn add_fallback_node(responses: Vec<(Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)>)
                     -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    add_composite_node(responses, fallback_node)
}

fn add_sequence_node(responses: Vec<(Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)>)
                     -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    add_composite_node(responses, sequence_node)
}

fn build_and_check_bt_with_composite(children: Vec<Arc<dyn BehaviorTreeNodeDefinition>>,
                                     composite_node_id: i32) {
    let tree_definition =
        common::one_off_root_tree(composite_node_id,
                                  children);

    common::check_builds_ok(tree_definition);
}

fn composite_node<F>(children: Vec<Arc<dyn BehaviorTreeNodeDefinition>>,
                     constructor: F)
                     -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)
    where F: Fn(i32, Vec<i32>) -> Arc<dyn BehaviorTreeNodeDefinition> {
    let ids: Vec<i32> = children
        .iter()
        .map(Deref::deref)
        .map(BehaviorTreeNodeDefinition::get_id)
        .map(Clone::clone)
        .collect();

    let fallback_id = *ids.iter().max().expect("Got empty child ids vec.") + 1;

    let mut response = Vec::new();

    response.extend(children);

    response.push(constructor(fallback_id, ids));

    (response, fallback_id)
}

fn composite_node_with_print_log_actions<F>(composite_node_provider: F,
                                            ids: Vec<i32>)
                                            -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)
    where F: Fn(Vec<Arc<dyn BehaviorTreeNodeDefinition>>) -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32)  {
    let mut nodes: Vec<Arc<dyn BehaviorTreeNodeDefinition>> = Vec::new();

    for id in ids {
        nodes.push(Arc::new(
            PrintLogActionNodeDefinition::new(
                id, "Hello!".to_owned())));
    }

    composite_node_provider(nodes)
}

fn fallback_node(children: Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
                 -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    composite_node(children,
                   |id, children_ids|
                       Arc::new(FallbackCompositeNodeDefinition::new(id, children_ids))
    )
}

fn fallback_node_with_print_log_actions(ids: Vec<i32>)
    -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    composite_node_with_print_log_actions(fallback_node, ids)
}

fn sequence_node(children: Vec<Arc<dyn BehaviorTreeNodeDefinition>>)
                 -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    composite_node(children,
                   |id, children_ids|
                       Arc::new(SequenceCompositeNodeDefinition::new(id, children_ids))
    )
}

fn sequence_node_with_print_log_actions(ids: Vec<i32>)
    -> (Vec<Arc<dyn BehaviorTreeNodeDefinition>>, i32) {
    composite_node_with_print_log_actions(sequence_node, ids)
}
