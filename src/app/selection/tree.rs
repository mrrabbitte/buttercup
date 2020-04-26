use std::collections::HashMap;

use crate::app::arguments::ArgumentDefinition;
use crate::app::common::addressable::Address;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDelegate, SelectionEdgeError};
use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDelegate, SelectionNodeError};
use crate::app::selection::tree::evaluation::SelectionTreeEvaluator;
use crate::app::transformations::Transformer;
use crate::app::transformations::transformer::TransformationRequest;
use crate::app::values::ValuesPayload;
use crate::app::content::commands::ContentCommandAddress;
use crate::app::selection::tree::decision::SelectionDecision;

pub mod evaluation;
pub mod decision;

pub struct SelectionTreeDefinition {

    id: i32,
    name: String,

}

pub struct SelectionTree {

    tenant_id: String,
    definition: SelectionTreeDefinition,
    evaluator: SelectionTreeEvaluator

}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                    payload: &ValuesPayload)
        -> Result<SelectionDecision, SelectionTreeError> {
        self.evaluator.select_commands(payload)
                .map(
                    |commands|
                        SelectionDecision::new(
                            self.definition.id, commands))

    }

}
