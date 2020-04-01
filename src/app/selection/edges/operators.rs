use crate::app::values::ValueHolder;

pub mod equality;
pub mod strings;

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

#[derive(Debug, Clone)]
pub enum RelationalOperatorError {

    UnsupportedValueType(ValueHolder)

}
impl RelationalOperator {

    pub fn evaluate(&self,
                     left: &ValueHolder,
                     right: &ValueHolder) -> Result<bool, RelationalOperatorError> {
        Result::Ok(true)
    }

}
// impl RelationalOperator {
//
//     pub fn evaluate(&self,
//                     left: &ValueHolder,
//                     right: &ValueHolder) -> Result<bool, RelationalOperatorError> {
//         return match &self {
//             RelationalOperator::Equals => {},
//             RelationalOperator::GreaterThan => {},
//             RelationalOperator::LessThan => {},
//             RelationalOperator::GreaterThanOrEquals => {},
//             RelationalOperator::LessThanOrEquals => {},
//             RelationalOperator::Contains => {},
//             RelationalOperator::StartsWith => {},
//             RelationalOperator::EndsWith => {},
//         };
//     }
//
//     fn get_evaluator(&self) -> &dyn RelationOperatorEvaluator {
//
//     }
//
// }

pub trait RelationOperatorEvaluator {

    fn evaluate(&self,
                left: &ValueHolder,
                right: &ValueHolder) -> Result<bool, RelationalOperatorError>;
    

}