use crate::app::conditions::{RelationalExpressionSpecification, ValuesPayloadPredicateSupplier};
use crate::app::values::{ValueHolder, ValuesPayload};

pub struct EqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

impl EqualsRelationalExpression {

    pub fn new(specification: RelationalExpressionSpecification) -> EqualsRelationalExpression {
        EqualsRelationalExpression { specification }
    }

}

impl ValuesPayloadPredicateSupplier for EqualsRelationalExpression {

    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        RelationalExpressions::build_predicate(
            self.specification,
            Box::new(
                |left, right| left.eq(right)))
    }

    fn get_value_names(&self) -> Vec<String> {
        self.specification.get_value_names()
    }
}

pub struct GreaterThanRelationalExpression {

    specification: RelationalExpressionSpecification

}

impl ValuesPayloadPredicateSupplier for GreaterThanRelationalExpression {
    fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        RelationalExpressions::build_predicate(
            self.specification,
            Box::new(
                |left, right| left.gt(right)))
    }

    fn get_value_names(&self) -> Vec<String> {
        self.specification.get_value_names()
    }
}


struct RelationalExpressions;

impl RelationalExpressions {

    fn build_predicate(specification: RelationalExpressionSpecification,
                       value_holders_predicate: Box<dyn Fn(&ValueHolder, &ValueHolder) -> bool>)
                       -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match specification {
            RelationalExpressionSpecification::NameAndName(first, second) =>
                Box::new(move |payload|
                    match (payload.get(&first), payload.get(&second)) {
                        (Some(left), Some(right)) =>
                            value_holders_predicate(left, right),
                        (_, _) => false
                    }),
            RelationalExpressionSpecification::NameAndLiteral(name, literal) =>
                Box::new(move |payload|
                    match payload.get(&name) {
                        Some(left) =>
                            value_holders_predicate(left, &literal),
                        _ => false
                    }),
            RelationalExpressionSpecification::LiteralAndName(literal, name) =>
                Box::new(move |payload|
                    match payload.get(&name) {
                        Some(right) =>
                            value_holders_predicate(&literal, right),
                        _ => false
                    }),
        }
    }

}


