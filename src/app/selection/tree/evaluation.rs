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
        let mut selected_command_ids: Vec<i32> = Vec::new();
        return match self.handle(&mut selected_command_ids, payload, &self.start_node) {
            Ok(_) => Result::Ok(selected_command_ids),
            Err(error) => Result::Err(error),
        };
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
              selected_command_ids: &mut Vec<i32>,
              payload: &ValuesPayload,
              current: &SelectionNode) -> Result<(), SelectionTreeError> {
        for address in current.get_outgoing_edges() {
            match self.get_edge(address) {
                Ok(edge) => {
                    match edge.can_pass(payload) {
                        Ok(can_pass) => {
                            if can_pass {
                                return match self.get_node(edge.get_next_selection_node()) {
                                    Ok(node) => {
                                        match node.select_content_command_id(payload) {
                                            Ok(command_id) =>
                                                selected_command_ids.push(*command_id),
                                            Err(error) =>
                                                return Result::Err(
                                                    SelectionTreeError::SelectionNodeError(error)),
                                        };
                                        self.handle(selected_command_ids, payload, node)
                                    },
                                    Err(error) => Result::Err(error),
                                };
                            }
                        },
                        Err(error) =>
                            return Result::Err(SelectionTreeError::SelectionEdgeError(error))
                    }
                },
                Err(error) => return Result::Err(error),
            }
        }
        Result::Ok(())
    }

}