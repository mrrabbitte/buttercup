use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDelegate};
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDelegate};
use crate::app::values::ValuesPayload;
use crate::app::selection::tree::SelectionTreeError;
use crate::app::selection::addressable::Address;

pub struct SelectionTreeEvaluator {

    start_node: SelectionNode,
    nodes: Vec<SelectionNode>,
    edges: Vec<SelectionEdge>

}

impl SelectionTreeEvaluator {

    pub fn select_commands(&self,
                           payload: &ValuesPayload) -> Result<Vec<i32>, SelectionTreeError> {
        let mut selection_nodes: Vec<&SelectionNode> = Vec::new();
        selection_nodes.push(&self.start_node);

        return Result::Ok(selected_command_ids);
    }


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

    fn get_next(&self, node: &SelectionNode) -> &SelectionNode {

    }

}