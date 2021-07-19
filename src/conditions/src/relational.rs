use buttercup_conditions_macros::RelationalExpression;
use buttercup_values::{ValueHolder, ValuesPayload};

use crate::{RelationalExpressionSpecification, ValuesPayloadPredicateSupplier};

use serde::{Deserialize, Serialize};

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(contains)]
pub struct ContainsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(ends_with)]
pub struct EndsWithRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(eq)]
pub struct EqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(ge)]
pub struct GreaterThanOrEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(gt)]
pub struct GreaterThanRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(is_in)]
pub struct IsInRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(le)]
pub struct LessThanOrEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(lt)]
pub struct LessThanRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(ne)]
pub struct NotEqualsRelationalExpression {

    specification: RelationalExpressionSpecification

}

#[derive(RelationalExpression, Serialize, Deserialize,
        Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
#[predicate(starts_with)]
pub struct StartsWithRelationalExpression {

    specification: RelationalExpressionSpecification

}
