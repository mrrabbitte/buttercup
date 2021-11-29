use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::action::subtree::ExecuteSubTreeActionNodeDefinition;
use buttercup_api::bts::composite::sequence::SequenceCompositeNodeDefinition;
use std::sync::Arc;

mod common;

#[test]
fn test_builds_subtree_node_correctly() {
    let subtree_id = 10;

    let tree_definition =
        common::one_off_root_tree(2,
                                  vec![
                                      Arc::new(
                                          ExecuteSubTreeActionNodeDefinition::new(
                                              2, subtree_id).into())
                                  ]);

    common::build_with_subtrees(tree_definition,
                                vec![
                                    common::one_off_root_tree_with_id(
                                        2,
                                        vec![
                                            Arc::new(
                                                PrintLogActionNodeDefinition::new(
                                                2, "I'm a subtree!".to_owned()).into()
                                            )
                                        ],
                                        subtree_id)])
        .expect("Expected the build to succeed!");
}

#[test]
fn test_builds_multiple_subtree_nodes_correctly() {
    let (first_subtree_id, second_subtree_id, third_subtree_id) = (10, 11, 12);

    let (definitions, sequence_node_id) = common::sequence_node(
        vec![
            Arc::new(
                ExecuteSubTreeActionNodeDefinition::new(
                    2, first_subtree_id).into()),
            Arc::new(
                ExecuteSubTreeActionNodeDefinition::new(
                    2, second_subtree_id).into()),
            Arc::new(
                ExecuteSubTreeActionNodeDefinition::new(
                    2, third_subtree_id).into())
        ]
    );

    let tree_definition =
        common::one_off_root_tree(sequence_node_id, definitions);

    common::build_with_subtrees(tree_definition,
                                vec![
                                    common::one_off_root_tree_with_id(
                                        2,
                                        vec![
                                            Arc::new(
                                                PrintLogActionNodeDefinition::new(
                                                    2, "I'm a first subtree!"
                                                        .to_owned()).into()
                                            )
                                        ],
                                        first_subtree_id),
                                    common::one_off_root_tree_with_id(
                                        2,
                                        vec![
                                            Arc::new(
                                                PrintLogActionNodeDefinition::new(
                                                    2, "I'm a second subtree!"
                                                        .to_owned()).into()
                                            )
                                        ],
                                        second_subtree_id),
                                    common::one_off_root_tree_with_id(
                                        2,
                                        vec![
                                            Arc::new(
                                                PrintLogActionNodeDefinition::new(
                                                    2, "I'm a third subtree!"
                                                        .to_owned()).into()
                                            )
                                        ],
                                        third_subtree_id)
                                ]
    )
        .expect("Expected the build to succeed!");
}