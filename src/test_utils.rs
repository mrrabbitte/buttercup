use std::sync::Arc;
use std::time::Duration;

use buttercup_agents::service::AgentService;
use buttercup_bts::context::BTNodeContextService;
use buttercup_bts::node::action::logging::PrintLogActionNode;
use buttercup_bts::node::action::subtree::ExecuteSubTreeActionNode;
use buttercup_bts::node::action::wait::WaitDurationActionNode;
use buttercup_bts::node::composite::sequence::SequenceCompositeNode;
use buttercup_bts::node::decorator::reactive::ReactiveConditionDecoratorNode;
use buttercup_bts::node::root::one_off::OneOffRootBTNode;
use buttercup_bts::node::root::to_first::{ToFirstErrorRootBTNode, ToFirstFailureRootBTNode};
use buttercup_bts::node::root::until_stopped::UntilStoppedRootBTNode;
use buttercup_bts::tree::{BehaviorTree, BehaviorTreeService};
use buttercup_conditions::{ConditionExpression, ConditionExpressionWrapper, LogicalExpression, RelationalExpression, RelationalExpressionSpecification};
use buttercup_conditions::relational::{EndsWithRelationalExpression, StartsWithRelationalExpression};
use buttercup_endpoints::endpoints::EndpointService;

pub fn build_test_agent_service(context_service: Arc<BTNodeContextService>) -> AgentService {
    let tree_service = Arc::new(BehaviorTreeService::default());

    add_test_trees(tree_service.as_ref());

    AgentService::new(context_service, tree_service).unwrap()
}

pub fn add_test_trees(bt_service: &BehaviorTreeService) {
    bt_service.insert(build_one_off_tree());
    bt_service.insert(build_until_stopped_tree());
    bt_service.insert(build_to_first_fail_tree());
    bt_service.insert(build_tree_with_subtree(bt_service.get_by_id(&1).unwrap()))
}


fn build_one_off_tree() -> BehaviorTree {
    BehaviorTree::new(
        1,
        OneOffRootBTNode::new(2,
                              PrintLogActionNode::new(3,
                                                      "Alive - one off".to_owned())
                                  .into())
            .into())
}

fn build_until_stopped_tree() -> BehaviorTree {
    BehaviorTree::new(
        2,
        UntilStoppedRootBTNode::new(3,
                                    SequenceCompositeNode::new(
                                        4, vec![
                                            WaitDurationActionNode::new(4,
                                                                        Duration::from_secs(3).into(),
                                            )
                                                .into(),
                                            PrintLogActionNode::new(5,
                                                                    "Until stopped".to_owned())
                                                .into()
                                        ]
                                    ).into()
        )
            .into())

}

fn build_to_first_fail_tree() -> BehaviorTree {
    BehaviorTree::new(
        3,
        ToFirstFailureRootBTNode::new(6,
                                      ReactiveConditionDecoratorNode::new(
                                          7,
                                          SequenceCompositeNode::new(
                                              8, vec![
                                                  WaitDurationActionNode::new(9,
                                                                              Duration::from_secs(3)
                                                                                  .into(),
                                                  )
                                                      .into(),
                                                  PrintLogActionNode::new(10,
                                                                          "To first failure - ignore errors".to_owned())
                                                      .into()
                                              ]
                                          ).into(),
                                          get_mock_condition_expression()
                                      ).into(),
                                      true
        )
            .into())
}

fn build_tree_with_subtree(subtree: Arc<BehaviorTree>) -> BehaviorTree {
    BehaviorTree::new(
        4,
        ToFirstErrorRootBTNode::new(11,
                                          SequenceCompositeNode::new(
                                              12, vec![
                                                  WaitDurationActionNode::new(13,
                                                                              Duration::from_secs(3).into(),
                                                  )
                                                      .into(),
                                                  PrintLogActionNode::new(14,
                                                                          "Subtree - here.".to_owned())
                                                      .into(),
                                                  ExecuteSubTreeActionNode::new(
                                                      15,
                                                      subtree
                                                  ).unwrap().into()
                                              ]
                                          ).into()
        )
            .into())
}

fn get_mock_condition_expression() -> ConditionExpressionWrapper {
    ConditionExpressionWrapper::new(
        ConditionExpression::LogicalExpression(
            Box::new(
                LogicalExpression::And(
                    vec![
                        ConditionExpression::RelationExpression(
                            RelationalExpression::EndsWith(
                                EndsWithRelationalExpression::new(
                                    RelationalExpressionSpecification::NameAndLiteral(
                                        "var_name_1".to_owned(),
                                        "elka".to_owned().into()
                                    )
                                )
                            )
                        ),
                        ConditionExpression::RelationExpression(
                            RelationalExpression::StartsWith(
                                StartsWithRelationalExpression::new(
                                    RelationalExpressionSpecification::NameAndLiteral(
                                        "var_name_1".to_owned(),
                                        "mira".to_owned().into()
                                    )
                                )
                            )
                        )
                    ]
                )
            )
        )
    )
}