use std::collections::HashMap;

use crate::app::arguments::ArgumentDefinition;
use crate::app::selection::addressable::Address;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDelegate};
use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDelegate};
use crate::app::transformations::transformer::TransformationRequest;

pub struct SelectionTreeDefinition {

    id: i32,
    name: String,
    project_definition_id: i32

}

pub struct SelectionTree {

    tenant_id: String,
    definition: SelectionTreeDefinition,
    argument_definitions: HashMap<String, ArgumentDefinition>,
    transformation_requests: Vec<TransformationRequest>,
    nodes: Vec<SelectionNode>,
    edges: Vec<SelectionEdge>

}

pub enum SelectionTreeError {

    MissingNode(SelectionNodeAddress),
    MissingEdge(SelectionEdgeAddress),
    SelectionNodeAddressIdMismatch(SelectionNodeAddress),
    SelectionEdgeAddressIdMismatch(SelectionEdgeAddress)

}

impl SelectionTree {

    fn get_node(&self,
                address: &SelectionNodeAddress) -> Result<&SelectionNode, SelectionTreeError> {
        return match self.nodes.get(*address.get_index()) {
            None => Result::Err(
                SelectionTreeError::MissingNode(address.clone())),
            Some(node) => {
                if !node.matches(address) {
                    return Result::Err(
                        SelectionTreeError::SelectionNodeAddressIdMismatch(
                            address.clone()));
                }
                return Result::Ok(node);
            }
        };
    }

    fn get_edge(&self,
                address: &SelectionEdgeAddress) -> Result<&SelectionEdge, SelectionTreeError> {
        return match self.edges.get(*address.get_index()) {
            None => Result::Err(
                SelectionTreeError::MissingEdge(address.clone())),
            Some(edge) => {
                if !edge.matches(address) {
                    return Result::Err(
                        SelectionTreeError::SelectionEdgeAddressIdMismatch(
                            address.clone()));
                }
                return Result::Ok(edge);
            }
        };
    }

}