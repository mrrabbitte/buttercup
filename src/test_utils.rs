use std::sync::Arc;

use buttercup_agents::service::AgentService;
use buttercup_bts::context::BTNodeContextService;
use buttercup_bts::node::action::logging::PrintLogActionNode;
use buttercup_bts::node::root::one_off::OneOffRootBTNode;
use buttercup_bts::tree::{BehaviorTree, BehaviorTreeService};
use buttercup_conditions::{
    ConditionExpression, LogicalExpression, RelationalExpression, RelationalExpressionSpecification};
use buttercup_conditions::relational::{EndsWithRelationalExpression, StartsWithRelationalExpression};
use buttercup_endpoints::endpoints::EndpointService;
use buttercup_bts::node::root::until_stopped::UntilStoppedRootBTNode;
use buttercup_bts::node::action::wait::WaitDurationActionNode;
use buttercup_bts::node::composite::sequence::SequenceCompositeNode;
use std::time::Duration;

pub fn build_test_agent_service(context_service: Arc<BTNodeContextService>) -> AgentService {
    let tree_service = Arc::new(BehaviorTreeService::default());

    add_test_trees(tree_service.as_ref());

    AgentService::new(context_service, tree_service).unwrap()
}

pub fn add_test_trees(bt_service: &BehaviorTreeService) {
    bt_service.insert(build_one_off_tree());
    bt_service.insert(build_until_stopped_tree());
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
                                                                        Duration::from_secs(3),
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

fn get_mock_condition_expression() -> ConditionExpression {
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

}