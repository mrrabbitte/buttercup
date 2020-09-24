use crate::app::conditions::{
    RelationalExpressionSpecification,
    ValuesPayloadPredicateSupplier};
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

    fn get(self) -> Box<dyn Fn(&ValuesPayload) -> bool> {
        match self.specification {
            RelationalExpressionSpecification::NameAndName(first, second) =>
                Box::new(move |payload|
                    match (payload.get(&first), payload.get(&second)) {
                        (Some(left), Some(right)) => left.eq(right),
                        (_, _) => false
                    }),
            RelationalExpressionSpecification::NameAndLiteral(name, literal) =>
                Box::new(move |payload|
                    match payload.get(&name) {
                        Some(left) => left.eq(&literal),
                        _ => false
                    }),
            RelationalExpressionSpecification::LiteralAndName(literal, name) =>
                Box::new(move |payload|
                    match payload.get(&name) {
                        Some(left) => literal.eq(left),
                        _ => false
                    }),
        }
    }

}

