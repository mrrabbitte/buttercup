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
        match current.select_content_command_id(payload) {
            Ok(command_id) =>
                selected_command_ids.push(*command_id),
            Err(error) =>
                return Result::Err(
                    SelectionTreeError::SelectionNodeError(error)),
        };
        for address in current.get_outgoing_edges() {
            match self.get_edge(address) {
                Ok(edge) => {
                    match edge.can_pass(payload) {
                        Ok(can_pass) => {
                            if can_pass {
                                return match self.get_node(edge.get_next_selection_node()) {
                                    Ok(node) =>
                                        self.handle(selected_command_ids, payload, node),
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
    use std::collections::HashMap;
    use std::hash::Hash;

    use chrono::Weekday;
    use num::{BigInt, FromPrimitive};
    use num_rational::BigRational;

    use crate::app::selection::edges::{SelectionEdgeDefinition, SelectionEdgeError, SelectionEdgeType};
    use crate::app::selection::edges::always::AlwaysTrueSelectionEdge;
    use crate::app::selection::edges::logical::{LogicalExpressionSelectionEdge, LogicalExpressionSelectionEdgeDetails};
    use crate::app::selection::edges::logical::conditions::{Condition, ConditionEvaluationError, ConditionValue};
    use crate::app::selection::edges::logical::expressions::{Expression, ExpressionAddress, ExpressionDefinition, ExpressionEvaluationError, NextExpressionAddressWithOperator};
    use crate::app::selection::edges::logical::operators::{LogicalOperator, RelationalOperator};
    use crate::app::selection::nodes::dictionary::{DictionaryNodeMapping,
                                                   DictionarySelectionNode,
                                                   DictionarySelectionNodeDetails};
    use crate::app::selection::nodes::SelectionNodeDefinition;
    use crate::app::selection::nodes::simple::{SimpleSelectionNode, SimpleSelectionNodeDetails};
    use crate::app::values::ValueHolder;
    use crate::app::values::wrappers::{WeekdayWrapper, Wrapper};

    use super::*;

    const FIRST_VALUE_NAME: &str = "firstValueName";
    const SECOND_VALUE_NAME: &str = "secondValueName";
    const THIRD_VALUE_NAME: &str = "thirdValueName";
    const FOURTH_VALUE_NAME: &str = "fourthValueName";
    const FIFTH_VALUE_NAME: &str = "fifthValueName";

    // #[test]
    // fn test_first_path() {
    //     let evaluator = build_evaluator();
    //     evaluator.select_commands();
    // }
    //
    // #[test]
    // fn test_second_path() {
    //     let evaluator = build_evaluator();
    //     evaluator.select_commands();
    // }
    //
    #[test]
    fn test_only_default_edge_match() {
        let evaluator = build_evaluator();
        let payload =
            build_payload(vec![
                (FIRST_VALUE_NAME.to_string(),
                 ValueHolder::DayOfWeek(
                     WeekdayWrapper::new(Weekday::Sat))),
                (SECOND_VALUE_NAME.to_string(),
                 ValueHolder::Decimal(BigRational::from_f64(0.3215421213).unwrap())),
                (THIRD_VALUE_NAME.to_string(),
                 ValueHolder::Decimal(BigRational::from_f64(0.3215421213).unwrap())),
                (FOURTH_VALUE_NAME.to_string(),
                 ValueHolder::Integer("11".parse::<BigInt>().unwrap())),
                (FIFTH_VALUE_NAME.to_string(),
                 ValueHolder::String("Borsm".to_string()))
            ]);
        println!("{:?}", evaluator.select_commands(&payload));
    }

    #[test]
    fn test_err_empty_payload() {
        let evaluator = build_evaluator();
        let payload =
            build_payload(vec![]);
        let result = evaluator.select_commands(&payload);
        assert_eq!(true, result.is_err());
        assert_eq!(SelectionTreeError::SelectionEdgeError(
            SelectionEdgeError::LogicalExpressionSelectionEdgeError(
                ExpressionEvaluationError::ConditionEvaluationError(
                    ConditionEvaluationError::DidNotFindLeftValue(
                        SECOND_VALUE_NAME.to_string())))),
                   result.err().unwrap());
    }

    fn build_evaluator() -> SelectionTreeEvaluator {
        let start_node: SelectionNode =
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        0, "Starting Node".to_string()),
                    vec![
                        SelectionEdgeAddress::new(0, 0),
                        SelectionEdgeAddress::new(1, 1)
                    ],
                    SimpleSelectionNodeDetails::new(0, 0)
                ));
        let nodes: Vec<SelectionNode> = vec![
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        1, "First After Condition Node".to_string()),
                    vec![SelectionEdgeAddress::new(2, 2)],
                    SimpleSelectionNodeDetails::new(1, 1)
                )),
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        2, "Second Default Node".to_string()),
                    vec![SelectionEdgeAddress::new(3, 3)],
                    SimpleSelectionNodeDetails::new(2, 2)
                )),
            SelectionNode::Dictionary(
                DictionarySelectionNode::new(
                    SelectionNodeDefinition::new(
                        3, "Third Dictionary Node".to_string()),
                    vec![],
                    DictionarySelectionNodeDetails::new(
                        3, 3,
                        FIRST_VALUE_NAME.to_string()),
                    DictionaryNodeMapping::new(3,
                                               build_map(
                                                   vec!{
                                                       (ValueHolder::DayOfWeek(
                                                           WeekdayWrapper::new(Weekday::Sat)),
                                                        4),
                                                       (ValueHolder::DayOfWeek(
                                                           WeekdayWrapper::new(Weekday::Sun)),
                                                        5)
                                                   })
                    )
                )),
            SelectionNode::Dictionary(
                DictionarySelectionNode::new(
                    SelectionNodeDefinition::new(
                        4, "Fourth Dictionary Node".to_string()),
                    vec![],
                    DictionarySelectionNodeDetails::new(
                        4, 6,
                        FIRST_VALUE_NAME.to_string()),
                    DictionaryNodeMapping::new(6,
                                               build_map(
                                                   vec!{
                                                       (ValueHolder::DayOfWeek(
                                                           WeekdayWrapper::new(Weekday::Sat)),
                                                        7),
                                                       (ValueHolder::DayOfWeek(
                                                           WeekdayWrapper::new(Weekday::Sun)),
                                                        8),
                                                       (ValueHolder::DayOfWeek(
                                                           WeekdayWrapper::new(Weekday::Mon)),
                                                        9)
                                                   })
                    ))
            )
        ];
        let edges: Vec<SelectionEdge> = vec![
            SelectionEdge::LogicalExpressionSelectionEdge(
                LogicalExpressionSelectionEdge::new(
                    SelectionEdgeDefinition::new(
                        0,
                        1,
                        SelectionEdgeType::LogicalExpressionSelectionEdge),
                    SelectionNodeAddress::new(1, 0),
                    LogicalExpressionSelectionEdgeDetails::new(0, 1),
                    vec![
                        Expression::new(
                            ExpressionDefinition::new(
                                1,  LogicalOperator::And),
                            vec![
                                Condition::new(0,
                                               SECOND_VALUE_NAME.to_string(),
                                               RelationalOperator::Equals,
                                               false,
                                               ConditionValue::Runtime(
                                                   THIRD_VALUE_NAME.to_string())),
                                Condition::new(1,
                                               THIRD_VALUE_NAME.to_string(),
                                               RelationalOperator::LessThan,
                                               false,
                                               ConditionValue::Static(
                                                   ValueHolder::Integer(
                                                       "10".parse::<BigInt>().unwrap())))
                            ],
                            Option::None)
                    ],
                    Expression::new(
                        ExpressionDefinition::new(
                            0,  LogicalOperator::And),
                        vec![
                            Condition::new(2,
                                           SECOND_VALUE_NAME.to_string(),
                                           RelationalOperator::Equals,
                                           false,
                                           ConditionValue::Runtime(
                                               THIRD_VALUE_NAME.to_string())),
                            Condition::new(3,
                                           THIRD_VALUE_NAME.to_string(),
                                           RelationalOperator::LessThan,
                                           false,
                                           ConditionValue::Static(
                                               ValueHolder::Integer(
                                                   "10".parse::<BigInt>().unwrap()))),
                            Condition::new(4,
                                           FOURTH_VALUE_NAME.to_string(),
                                           RelationalOperator::GreaterThanOrEquals,
                                           true,
                                           ConditionValue::Static(
                                               ValueHolder::Integer(
                                                   "10".parse::<BigInt>().unwrap())))
                        ],
                        Option::Some(
                            NextExpressionAddressWithOperator::new(
                                ExpressionAddress::new(1, 0),
                                LogicalOperator::Or))
                    )
                )),
            SelectionEdge::AlwaysTrueSelectionEdge(
                AlwaysTrueSelectionEdge::new(
                    SelectionEdgeDefinition::new(
                        1, 2,
                        SelectionEdgeType::AlwaysTrueSelectionEdge),
                    SelectionNodeAddress::new(2, 1)
                )),
            SelectionEdge::LogicalExpressionSelectionEdge(
                LogicalExpressionSelectionEdge::new(
                    SelectionEdgeDefinition::new(
                        2,
                        3,
                        SelectionEdgeType::LogicalExpressionSelectionEdge),
                    SelectionNodeAddress::new(3, 2),
                    LogicalExpressionSelectionEdgeDetails::new(0, 1),
                    vec![],
                    Expression::new(
                        ExpressionDefinition::new(
                            2,  LogicalOperator::And),
                        vec![
                            Condition::new(5,
                                           FIFTH_VALUE_NAME.to_string(),
                                           RelationalOperator::Contains,
                                           false,
                                           ConditionValue::Static(
                                               ValueHolder::String(String::from("ski"))
                                           ))
                        ],
                        Option::None
                    )
                )),
            SelectionEdge::AlwaysTrueSelectionEdge(
                AlwaysTrueSelectionEdge::new(
                    SelectionEdgeDefinition::new(
                        3, 4,
                        SelectionEdgeType::AlwaysTrueSelectionEdge),
                    SelectionNodeAddress::new(4, 3)
                ))
        ];
        SelectionTreeEvaluator {
            start_node,
            nodes,
            edges
        }
    }

    fn build_map<K,V>(entries: Vec<(K, V)>) -> HashMap<K, V>
        where K: Hash + Eq {
        let mut ret = HashMap::new();
        for entry in entries {
            ret.insert(entry.0, entry.1);
        }
        ret
    }

    fn build_payload(values: Vec<(String, ValueHolder)>) -> ValuesPayload {
        ValuesPayload::new(build_map(values))
    }

}