use buttercup_api::bts::decorator::condition::ConditionDecoratorNodeDefinition;
use buttercup_conditions::{ConditionExpression, LogicalExpression, RelationalExpression, RelationalExpressionSpecification};
use buttercup_conditions::relational::EqualsRelationalExpression;

use serde_json;
use buttercup_api::bts::BehaviorTreeDefinition;
use buttercup_api::bts::root::OneOffRootBTNodeDefinition;

const FIRST_VALUE_NAME: &str = "first_value_name";
const FIRST_VALUE: &str = "first_value";

const SECOND_VALUE_NAME: &str = "second_value_name";
const SECOND_VALUE: &str = "second_value";

const THIRD_VALUE_NAME: &str = "third_value_name";
const THIRD_VALUE: u8 = 2;

#[test]
fn test_serde_conditions() {
    let node = ConditionDecoratorNodeDefinition::new(
        2,
        3,
        simple_expression());

    let read: ConditionDecoratorNodeDefinition =
        serde_json::from_str(serde_json::to_string(&node).unwrap().as_str()).unwrap();

    assert_eq!(node, read);
}


fn simple_expression() -> ConditionExpression {
    ConditionExpression::LogicalExpression(
        Box::new(
            LogicalExpression::Or(
                vec![
                    ConditionExpression::RelationExpression(
                        RelationalExpression::Equals(
                            EqualsRelationalExpression::new(
                                RelationalExpressionSpecification::NameAndName(
                                    FIRST_VALUE_NAME.to_owned(), SECOND_VALUE_NAME.to_owned()
                                )
                            )
                        )
                    ),
                    ConditionExpression::RelationExpression(
                        RelationalExpression::Equals(
                            EqualsRelationalExpression::new(
                                RelationalExpressionSpecification::NameAndName(
                                    THIRD_VALUE_NAME.to_owned(), FIRST_VALUE_NAME.to_owned()
                                )
                            )
                        )
                    )
                ]
            )
        )
    )
}