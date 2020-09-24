use crate::app::conditions::value::EqualsRelationalExpression;
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod value;

pub enum ConditionExpression {

    RelationExpression(RelationalExpression),
    LogicalExpression(Box<LogicalExpression>)

}

pub struct ConditionExpressionWrapper {

    predicate: Box<dyn Fn(&ValuesPayload) -> bool>

}

impl ConditionExpressionWrapper {

    pub fn new(condition: ConditionExpression) -> ConditionExpressionWrapper {
        ConditionExpressionWrapper {
            predicate: condition.get()
        }
    }

    pub fn unpack(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        self.predicate
    }

}

pub enum LogicalExpression {

    And(Vec<ConditionExpression>),
    Or(Vec<ConditionExpression>),
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

    fn get(self) -> Box<dyn Fn(&ValuesPayload) -> bool>;

}

impl ValuesPayloadPredicateSupplier for ConditionExpression {

    fn get(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            ConditionExpression::RelationExpression(expr) => expr.get(),
            ConditionExpression::LogicalExpression(expr) => expr.get()
        }
    }

}

impl ValuesPayloadPredicateSupplier for LogicalExpression {
    fn get(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            LogicalExpression::And(expressions) => {
                let expr_funcs: Vec<Box<dyn Fn(&ValuesPayload) -> bool>> = to_funcs(expressions);
                Box::new(move |payload|
                    {
                        for expr in &expr_funcs {
                            if !expr(payload) {
                                return false;
                            }
                        }
                        return true;
                    }
                )
            },
            LogicalExpression::Or(expressions) => {
                let expr_funcs: Vec<Box<dyn Fn(&ValuesPayload) -> bool>> = to_funcs(expressions);
                Box::new(move |payload|
                    {
                        for expr in &expr_funcs {
                            if expr(payload) {
                                return true;
                            }
                        }
                        return false;
                    })
            },
            LogicalExpression::Not(expr) => {
                let expr_func = expr.get();
                Box::new(move |payload| !expr_func(payload))
            }

        }
    }
}

impl ValuesPayloadPredicateSupplier for RelationalExpression {
    fn get(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            RelationalExpression::Equals(equals) => equals.get()
        }
    }
}

fn to_funcs(expressions: Vec<ConditionExpression>) -> Vec<Box<dyn Fn(&ValuesPayload) -> bool>> {
    let mut ret = Vec::new();
    for expr in expressions {
        ret.push(expr.get());
    }
    ret
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use num::bigint::BigInt;
    use num::FromPrimitive;

    use crate::app::values::ValueHolder;

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
        );

        let predicate = ConditionExpressionWrapper::new(condition).unpack();

        assert_eq!(predicate(&first_values_payload()), true);
        assert_eq!(predicate(&second_values_payload()), false);
    }

    #[test]
    fn test_evaluates_correctly_for_equals_name_literal() {
        let condition = ConditionExpression::LogicalExpression(
            Box::new(
                LogicalExpression::And(
                    vec![
                        ConditionExpression::RelationExpression(
                            RelationalExpression::Equals(
                                EqualsRelationalExpression::new(
                                    RelationalExpressionSpecification::LiteralAndName(
                                        ValueHolder::String(FIRST_VALUE.to_owned()),
                                        FIRST_VALUE_NAME.to_owned()
                                    )
                                )
                            )
                        ),
                        ConditionExpression::RelationExpression(
                            RelationalExpression::Equals(
                                EqualsRelationalExpression::new(
                                    RelationalExpressionSpecification::LiteralAndName(
                                        ValueHolder::String(FIRST_VALUE.to_owned()),
                                        SECOND_VALUE_NAME.to_owned()
                                    )
                                )
                            )
                        ),
                        ConditionExpression::RelationExpression(
                            RelationalExpression::Equals(
                                EqualsRelationalExpression::new(
                                    RelationalExpressionSpecification::NameAndLiteral(
                                        THIRD_VALUE_NAME.to_owned(),
                                        ValueHolder::Integer(BigInt::from(THIRD_VALUE))
                                    )
                                )
                            )
                        )
                    ]
                )
            )
        );

        let predicate = ConditionExpressionWrapper::new(condition).unpack();

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
