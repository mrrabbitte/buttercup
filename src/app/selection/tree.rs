use std::collections::HashMap;

use crate::app::arguments::ArgumentDefinition;
use crate::app::selection::addressable::Address;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDelegate, SelectionEdgeError};
use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDelegate, SelectionNodeError};
use crate::app::selection::tree::evaluation::SelectionTreeEvaluator;
use crate::app::transformations::Transformer;
use crate::app::transformations::transformer::TransformationRequest;
use crate::app::values::ValuesPayload;

pub mod evaluation;

pub struct SelectionTreeDefinition {

    id: i32,
    name: String,
    project_definition_id: i32

}

pub struct SelectionTree {

    tenant_id: String,
    definition: SelectionTreeDefinition,
    evaluator: SelectionTreeEvaluator

}

pub enum SelectionTreeError {

    SelectionNodeError(SelectionNodeError),
    SelectionEdgeError(SelectionEdgeError),
    MissingNode(SelectionNodeAddress),
    MissingEdge(SelectionEdgeAddress),
    SelectionNodeAddressIdMismatch(SelectionNodeAddress),
    SelectionEdgeAddressIdMismatch(SelectionEdgeAddress)

}

impl SelectionTree {

    pub fn evaluate(&self,
                    payload: &ValuesPayload) -> Result<Vec<i32>, SelectionTreeError> {
        self.evaluator.select_commands(payload)
    }

}