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

    pub fn new(start_node: SelectionNode,
               nodes: Vec<SelectionNode>,
               edges: Vec<SelectionEdge>) -> SelectionTreeEvaluator {
        SelectionTreeEvaluator {
            start_node,
            nodes,
            edges
        }
    }

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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::app::selection::nodes::simple::{SimpleSelectionNode, SimpleSelectionNodeDetails};
    use crate::app::selection::nodes::SelectionNodeDefinition;
    use crate::app::selection::nodes::dictionary::{DictionaryNodeMapping,
                                                   DictionarySelectionNode,
                                                   DictionarySelectionNodeDetails};
    use crate::app::values::ValueHolder;
    use crate::app::values::wrappers::{WeekdayWrapper, Wrapper};
    use chrono::Weekday;
    use crate::app::selection::edges::logical::{LogicalExpressionSelectionEdge, LogicalExpressionSelectionEdgeDetails};
    use crate::app::selection::edges::{SelectionEdgeDefinition, SelectionEdgeType};

    const FIRST_DICT_VALUE_NAME: &str = "FirstValueName";

    #[test]
    fn test_first_path() {
        let evaluator = build_evaluator();
        evaluator.select_commands();
    }

    #[test]
    fn test_second_path() {
        let evaluator = build_evaluator();
        evaluator.select_commands();
    }

    #[test]
    fn test_nothing_when_no_edge_matches() {
        let evaluator = build_evaluator();
        evaluator.select_commands();
    }

    fn build_evaluator() -> SelectionTreeEvaluator {
        let start_node: SelectionNode =
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(0,
                                                 "Starting Node".to_string()),
                    vec![],
                    SimpleSelectionNodeDetails::new(0, 0)
                ));
        let nodes: Vec<SelectionNode> = vec![
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        1, "First After Condition Node".to_string()),
                    vec![],
                    SimpleSelectionNodeDetails::new(1, 1)
                )),
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        2, "Second Default Node".to_string()),
                    vec![],
                    SimpleSelectionNodeDetails::new(2, 2)
                )),
            SelectionNode::Dictionary(
                DictionarySelectionNode::new(
                    SelectionNodeDefinition::new(
                        3, "Second Default Node".to_string()),
                    vec![],
                    DictionarySelectionNodeDetails::new(
                        3, 3,
                        FIRST_DICT_VALUE_NAME.to_string()),
                    DictionaryNodeMapping::new(3,
                                               hashmap!{
                                               ValueHolder::DayOfWeek(WeekdayWrapper::new(Weekday::Sat)) => 4,
                                               ValueHolder::DayOfWeek(WeekdayWrapper::new(Weekday::Sun)) => 5
                                               })
                )),
            SelectionNode::Dictionary(
                DictionarySelectionNode::new(
                    SelectionNodeDefinition::new(
                        4, "Second Default Node".to_string()),
                    vec![],
                    DictionarySelectionNodeDetails::new(
                        4, 6,
                        FIRST_DICT_VALUE_NAME.to_string()),
                    DictionaryNodeMapping::new(6,
                                               hashmap!{
                                               ValueHolder::String("FirstVal".to_string()) => 7,
                                               ValueHolder::String("FirstVal".to_string()) => 8,
                                               ValueHolder::String("FirstVal".to_string()) => 9
                                               })
                ))
        ];
        let edges: Vec<SelectionEdge> = vec![
            SelectionEdge::LogicalExpressionSelectionEdge(
                LogicalExpressionSelectionEdge::new(
                    SelectionEdgeDefinition::new(
                        1,
                        1,
                        SelectionEdgeType::LogicalExpressionSelectionEdge),
                    SelectionNodeAddress::new(1, 0),
                    LogicalExpressionSelectionEdgeDetails::new()
                ))
        ];
        SelectionTreeEvaluator {
            start_node,
            nodes,
            edges
        }
    }

}