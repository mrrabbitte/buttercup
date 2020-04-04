use crate::app::selection::edges::logical::operators::LogicalOperator;
use crate::app::selection::addressable::Address;

pub struct ExpressionDefinition {

    id: i32,
    internal_logical_operator: LogicalOperator

}

pub struct Expression {

    definition: ExpressionDefinition

}

pub struct ExpressionAddress {

    id: i32,
    index: usize

}

impl Address for ExpressionAddress {

    fn new(id: i32, index: usize) -> Self {
        ExpressionAddress {
            id,
            index
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }

    fn get_index(&self) -> &usize {
        &self.index
    }

}