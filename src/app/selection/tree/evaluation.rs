use crate::app::selection::addressable::Address;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDelegate};
use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDelegate, SelectionNodeError};
use crate::app::selection::tree::SelectionTreeError;
use crate::app::values::ValuesPayload;

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
        match self.handle(&mut selection_nodes, payload, start_node) {
            Ok(_) => {
                let mut selected_command_ids: Vec<i32> = Vec::new();
                for node in selection_nodes {
                    match node.select_content_command_id(payload) {
                        Ok(command_id) => selected_command_ids.push(*command_id),
                        Err(error) =>
                            return Result::Err(SelectionTreeError::SelectionNodeError(error)),
                    }
                }
                return Result::Ok(selected_command_ids);
            },
            Err(error) => return Result::Err(error),
        }
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

    fn handle(&self,
              to_be_evaluated: &mut Vec<&SelectionNode>,
              payload: &ValuesPayload,
              current: &SelectionNode) -> Result<(), SelectionTreeError> {
        for address in current.get_outgoing_edges() {
            match self.get_edge(address) {
                Ok(edge) => {
                    if edge.can_pass(payload) {
                        match self.get_node(edge.get_next_selection_node()) {
                            Ok(node) => {
                                to_be_evaluated.push(node);
                                return self.handle(to_be_evaluated, payload, node);
                            },
                            Err(error) => Result::Err(error),
                        }
                    }
                },
                Err(error) => return Result::Err(error),
            }
        }
        Result::Ok(())
    }

}