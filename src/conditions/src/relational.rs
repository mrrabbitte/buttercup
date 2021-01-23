use crate::app::conditions::{RelationalExpressionSpecification, ValuesPayloadPredicateSupplier};
use crate::app::values::{ValueHolder, ValuesPayload};

use buttercup_macros::RelationalExpression;

#[derive(RelationalExpression)]
#[predicate(contains)]
pub struct ContainsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression)]
#[predicate(ends_with)]
pub struct EndsWithRelationalExpression {

    specification: RelationalExpressionSpecification

}

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
#[predicate(is_in)]
pub struct IsInRelationalExpression {

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

#[derive(RelationalExpression)]
#[predicate(starts_with)]
pub struct StartsWithRelationalExpression {

    specification: RelationalExpressionSpecification

}
