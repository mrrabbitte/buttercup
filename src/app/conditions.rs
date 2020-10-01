use std::collections::HashSet;
use std::iter::FromIterator;

use crate::app::conditions::relational::{EqualsRelationalExpression, GreaterThanRelationalExpression};
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod relational;

pub enum ConditionExpression {

    RelationExpression(RelationalExpression),
    LogicalExpression(Box<LogicalExpression>)

}

pub struct ConditionExpressionWrapper {

    predicate: Box<dyn Fn(&ValuesPayload) -> bool>,
    value_names: HashSet<String>

}

impl ConditionExpressionWrapper {

    pub fn new(condition: ConditionExpression) -> ConditionExpressionWrapper {
        let value_names = HashSet::from_iter(condition.get_value_names());
        ConditionExpressionWrapper {
            predicate: condition.get_predicate(),
            value_names
        }
    }

    pub fn unpack(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        self.predicate
    }

    pub fn get_value_names_cloned(&self) -> HashSet<String> {
        self.value_names.iter().cloned().collect()
    }

}

pub enum LogicalExpression {

    And(Vec<ConditionExpression>),
    Or(Vec<ConditionExpression>),
    Not(ConditionExpression)

}

pub enum RelationalExpression {

    Equals(EqualsRelationalExpression),
    GreaterThan(GreaterThanRelationalExpression)

}

pub enum RelationalExpressionSpecification {

    NameAndName(String, String),
    NameAndLiteral(String, ValueHolder),
    LiteralAndName(ValueHolder, String)

}

impl RelationalExpressionSpecification {

    pub fn get_value_names(&self) -> Vec<String> {
        match self {
            RelationalExpressionSpecification::NameAndName(first, second) =>
                vec![first.clone(), second.clone()],
            RelationalExpressionSpecification::NameAndLiteral(name, _) =>
                vec![name.clone()],
            RelationalExpressionSpecification::LiteralAndName(_, name) =>
                vec![name.clone()]
        }
    }

}

pub trait ValuesPayloadPredicateSupplier {

    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool>;
    fn get_value_names(&self) -> Vec<String>;

}

impl ValuesPayloadPredicateSupplier for ConditionExpression {

    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            ConditionExpression::RelationExpression(expr) => expr.get_predicate(),
            ConditionExpression::LogicalExpression(expr) => expr.get_predicate()
        }
    }

    fn get_value_names(&self) -> Vec<String> {
        match self {
            ConditionExpression::RelationExpression(expr) =>
                expr.get_value_names(),
            ConditionExpression::LogicalExpression(expr) =>
                expr.get_value_names()
        }
    }

}

impl ValuesPayloadPredicateSupplier for LogicalExpression {
    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            LogicalExpression::And(expressions) => {
                let expr_funcs: Vec<Box<dyn Fn(&ValuesPayload) -> bool>> =
                    to_predicates(expressions);
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
                let expr_funcs: Vec<Box<dyn Fn(&ValuesPayload) -> bool>> =
                    to_predicates(expressions);
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
                let expr_func = expr.get_predicate();
                Box::new(move |payload| !expr_func(payload))
            }

        }
    }

    fn get_value_names(&self) -> Vec<String> {
        match self {
            LogicalExpression::And(expressions) =>
                to_value_names(expressions),
            LogicalExpression::Or(expressions) =>
                to_value_names(expressions),
            LogicalExpression::Not(expr) => expr.get_value_names()
        }
    }
}

impl ValuesPayloadPredicateSupplier for RelationalExpression {
    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self {
            RelationalExpression::Equals(expr) =>
                expr.get_predicate(),
            RelationalExpression::GreaterThan(expr) =>
                expr.get_predicate()
        }
    }

    fn get_value_names(&self) -> Vec<String> {
        match self {
            RelationalExpression::Equals(expr) =>
                expr.get_value_names(),
            RelationalExpression::GreaterThan(expr) =>
                expr.get_value_names()
        }
    }
}

fn to_predicates(expressions: Vec<ConditionExpression>)
                 -> Vec<Box<dyn Fn(&ValuesPayload) -> bool>> {
    let mut ret = Vec::new();
    for expr in expressions {
        ret.push(expr.get_predicate());
    }
    ret
}

fn to_value_names(expressions: &Vec<ConditionExpression>) -> Vec<String> {
    let mut ret = Vec::new();
    for expr in expressions {
        ret.extend(expr.get_value_names());
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
