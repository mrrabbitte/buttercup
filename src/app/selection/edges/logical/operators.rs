use serde::{Deserialize, Serialize};

use crate::app::selection::edges::logical::operators::strings::StringOperators;
use crate::app::values::ValueHolder;

pub mod strings;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalOperator {

    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    Contains,
    StartsWith,
    EndsWith

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalOperatorError {

    UnsupportedValueTypeForOperator(ValueHolder, RelationalOperator),
    UnsupportedOperatorForStrings(RelationalOperator)

}

impl RelationalOperator {

    pub fn evaluate(&self,
                    left: &ValueHolder,
                    right: &ValueHolder) -> Result<bool, RelationalOperatorError> {
        return match &self {
            RelationalOperator::Equals => Result::Ok(left.eq(right)),
            RelationalOperator::GreaterThan => Result::Ok(left.gt(right)),
            RelationalOperator::LessThan => Result::Ok(left.lt(right)),
            RelationalOperator::GreaterThanOrEquals => Result::Ok(left.ge(right)),
            RelationalOperator::LessThanOrEquals => Result::Ok(left.le(right)),
            RelationalOperator::Contains => StringOperators::handle(self, left, right),
            RelationalOperator::StartsWith => StringOperators::handle(self, left, right),
            RelationalOperator::EndsWith => StringOperators::handle(self, left, right),
        };
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogicalOperator {

    And,
    Or

}