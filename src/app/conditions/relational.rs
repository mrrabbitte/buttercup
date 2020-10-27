use crate::app::conditions::{RelationalExpressionSpecification, ValuesPayloadPredicateSupplier};
use crate::app::values::{ValueHolder, ValuesPayload};

use buttercup_macros::RelationalExpression;

#[derive(RelationalExpression)]
#[predicate(eq)]
pub struct EqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(ge)]
pub struct GreaterThanOrEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(gt)]
pub struct GreaterThanRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(le)]
pub struct LessThanOrEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(lt)]
pub struct LessThanRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(ne)]
pub struct NotEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}
