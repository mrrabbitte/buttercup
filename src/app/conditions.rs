use crate::app::conditions::value::EqualsRelationalExpression;
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod value;

pub enum ConditionExpression {

    RelationExpression(RelationalExpression),
    LogicalExpression(Box<LogicalExpression>)

}

pub enum LogicalExpression {

    And(ConditionExpression, ConditionExpression),
    Or(ConditionExpression, ConditionExpression),
    Not(ConditionExpression)

}

pub enum RelationalExpression {

    Equals(EqualsRelationalExpression)

}

pub enum RelationalExpressionSpecification {

    NameAndName(String, String),
    NameAndLiteral(String, ValueHolder),
    LiteralAndName(ValueHolder, String)

}

pub trait ValuesPayloadPredicateSupplier {

    fn get(&self) -> Box<dyn Fn(&ValuesPayload) -> bool + '_>;

}

impl ValuesPayloadPredicateSupplier for ConditionExpression {

    fn get(&self) -> Box<dyn Fn(&ValuesPayload) -> bool + '_> {
        match self {
            ConditionExpression::RelationExpression(expr) => expr.get(),
            ConditionExpression::LogicalExpression(expr) => expr.get()
        }
    }

}

impl ValuesPayloadPredicateSupplier for LogicalExpression {
    fn get(&self) -> Box<dyn Fn(&ValuesPayload) -> bool + '_> {
        match self {
            LogicalExpression::And(first, second) =>
                Box::new(move |payload| first.get()(payload) && second.get()(payload)),
            LogicalExpression::Or(first, second) =>
                Box::new(move |payload| first.get()(payload) || second.get()(payload)),
            LogicalExpression::Not(expr) =>
                Box::new(move |payload| !expr.get()(payload))
        }
    }
}

impl ValuesPayloadPredicateSupplier for RelationalExpression {
    fn get(&self) -> Box<dyn Fn(&ValuesPayload) -> bool+ '_> {
        match self {
            RelationalExpression::Equals(equals) => equals.get()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use num::FromPrimitive;
    use crate::app::values::ValueHolder;

    use num::bigint::BigInt;

    use super::*;

    const FIRST_VALUE_NAME: &str = "first_value_name";
    const FIRST_VALUE: &str = "first_value";

    const SECOND_VALUE_NAME: &str = "second_value_name";
    const SECOND_VALUE: &str = "second_value";

    const THIRD_VALUE_NAME: &str = "third_value_name";
    const THIRD_VALUE: u8 = 2;

    #[test]
    fn test_evaluates_correctly_for_equals_name_name() {
        let condition = ConditionExpression::LogicalExpression(
            Box::new(
                LogicalExpression::Or(
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
                )
            )
        );

        let predicate = condition.get();

        assert_eq!(predicate(&first_values_payload()), true);
        assert_eq!(predicate(&second_values_payload()), false);
    }

    fn first_values_payload() -> ValuesPayload {
        let mut values = HashMap::new();
        values.insert(
            FIRST_VALUE_NAME.to_owned(),
            ValueHolder::String(FIRST_VALUE.to_owned()));
        values.insert(
            SECOND_VALUE_NAME.to_owned(),
            ValueHolder::String(FIRST_VALUE.to_owned()));
        values.insert(
            THIRD_VALUE_NAME.to_owned(),
            ValueHolder::Integer(BigInt::from(THIRD_VALUE)));
        ValuesPayload::new(values)
    }

    fn second_values_payload() -> ValuesPayload {
        let mut values = HashMap::new();
        values.insert(
            FIRST_VALUE_NAME.to_owned(),
            ValueHolder::String(FIRST_VALUE.to_owned()));
        values.insert(
            SECOND_VALUE_NAME.to_owned(),
            ValueHolder::String(SECOND_VALUE.to_owned()));
        values.insert(
            THIRD_VALUE_NAME.to_owned(),
            ValueHolder::Integer(BigInt::from(THIRD_VALUE)));
        ValuesPayload::new(values)
    }

}
