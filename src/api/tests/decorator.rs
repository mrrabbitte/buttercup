use std::sync::Arc;

use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::decorator::condition::ConditionDecoratorNodeDefinition;
use buttercup_conditions::ConditionExpression;

mod common;

#[test]
fn test_builds_ok_with_single_decorator_node() {
    let tree_definition =
        common::one_off_root_tree(1,
                                  vec![
                                      Arc::new(
                                          ConditionDecoratorNodeDefinition::new(
                                              1, 2,
                                              ConditionExpression::ConstantExpression(
                                                  true))),
                                      Arc::new(
                                          PrintLogActionNodeDefinition::new(
                                              2,
                                              "I'm a decorator child node.".to_owned()))
                                  ]);

    common::check_builds_ok(tree_definition);
}