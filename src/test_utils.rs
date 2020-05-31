use std::collections::HashMap;

use chrono::Weekday;
use num::BigInt;
use num_rational::BigRational;

use crate::app::arguments::{ArgumentDefinition, ArgumentsExtractor};
use crate::app::common::addressable::Address;
use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutor};
use crate::app::content::definitions::ContentType;
use crate::app::pipeline::core::ContentPipeline;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeAddress, SelectionEdgeDefinition, SelectionEdgeType};
use crate::app::selection::edges::always::AlwaysTrueSelectionEdge;
use crate::app::selection::edges::logical::{LogicalExpressionSelectionEdge, LogicalExpressionSelectionEdgeDetails};
use crate::app::selection::edges::logical::conditions::{Condition, ConditionValue};
use crate::app::selection::edges::logical::expressions::{Expression, ExpressionAddress, ExpressionDefinition, NextExpressionAddressWithOperator};
use crate::app::selection::edges::logical::operators::{LogicalOperator, RelationalOperator};
use crate::app::selection::nodes::{SelectionNode, SelectionNodeAddress, SelectionNodeDefinition};
use crate::app::selection::nodes::dictionary::{DictionaryNodeMapping, DictionarySelectionNode, DictionarySelectionNodeDetails};
use crate::app::selection::nodes::simple::{SimpleSelectionNode, SimpleSelectionNodeDetails};
use crate::app::selection::tree::{SelectionTree, SelectionTreeDefinition};
use crate::app::selection::tree::evaluation::SelectionTreeEvaluator;
use crate::app::transformations::di::DiInputTransformation;
use crate::app::transformations::mono::MonoInputTransformation;
use crate::app::transformations::Transformer;
use crate::app::transformations::transformer::{DoubleInputTransformationDefinition, SingleInputTransformationDefinition, Transformation, TransformationDefinition, TransformationRequest, TransformationType};
use crate::app::values::{ValueHolder, ValueType};
use crate::app::values::extractors::ValueExtractionPolicy;
use crate::app::values::wrappers::{WeekdayWrapper, Wrapper};
use std::hash::Hash;
use crate::app::files::FileService;
use crate::app::files::path::FilesPathService;

pub struct TestUtils;

impl TestUtils {

    const TENANT_ID: &'static str = "tenant_id_1";
    const BASE_DIR: &'static str = "/content/";
    const BASE_PATH: &'static str = "/content/";

    pub fn test_file_service() -> FileService {
        let mut tenant_paths = HashMap::new();
        tenant_paths.insert(TestUtils::TENANT_ID.to_owned(), "t_1/");
        FileService::new(
            root_path: TestUtils::BASE_PATH,
            root_dir: TestUtils::BASE_DIR,
            files_path_service: FilesPathService::new(
                TestUtils::BASE_PATH, tenant_paths)
        )
    }

    pub fn test_pipeline() -> ContentPipeline {
        let tenant_id = TENANT_ID.to_owned();
        let mut argument_definitions = HashMap::new();
        argument_definitions.insert("dayOfWeekArg".to_owned(),
                                    ArgumentDefinition::new(1,
                                                            "dayOfWeekArg".to_owned(),
                                                            ValueType::DayOfWeek,
                                                            ValueExtractionPolicy::Lax,
                                                            1));
        argument_definitions.insert("decimalArg".to_owned(),
                                    ArgumentDefinition::new(2,
                                                            "decimalArg".to_owned(),
                                                            ValueType::Decimal,
                                                            ValueExtractionPolicy::Lax,
                                                            1));
        argument_definitions.insert("geoArg".to_owned(),
                                    ArgumentDefinition::new(3,
                                                            "geoArg".to_owned(),
                                                            ValueType::GeoCoordinates,
                                                            ValueExtractionPolicy::Lax,
                                                            1));
        argument_definitions.insert("dateTimeArg".to_owned(),
                                    ArgumentDefinition::new(4,
                                                            "dateTimeArg".to_owned(),
                                                            ValueType::LocalDateTime,
                                                            ValueExtractionPolicy::Lax,
                                                            1));
        argument_definitions.insert("zoneArg".to_owned(),
                                    ArgumentDefinition::new(5,
                                                            "zoneArg".to_owned(),
                                                            ValueType::TimeZone,
                                                            ValueExtractionPolicy::Lax,
                                                            1));
        let mut transformation_requests = Vec::new();
        transformation_requests.push(TransformationRequest::new(
            TransformationDefinition::new(1,
                                          TransformationType::SingleInput,
                                          "tzForGeoArg".to_owned()),
            Transformation::Mono(
                SingleInputTransformationDefinition::new(
                    1,
                    "geoArg".to_owned(),
                    MonoInputTransformation::FindTimeZoneFromGeoCoordinates))
        ));
        transformation_requests.push(TransformationRequest::new(
            TransformationDefinition::new(2,
                                          TransformationType::SingleInput,
                                          "dayOfWeekForDateTimeArg".to_owned()),
            Transformation::Mono(
                SingleInputTransformationDefinition::new(
                    2,
                    "dateTimeArg".to_owned(),
                    MonoInputTransformation::DayOfWeekFromDateTimeRetrieval))
        ));
        transformation_requests.push(TransformationRequest::new(
            TransformationDefinition::new(3,
                                          TransformationType::DoubleInput,
                                          "zonedDateTimeForPosition".to_owned()),
            Transformation::Bi(
                DoubleInputTransformationDefinition::new(
                    3,
                    "dateTimeArg".to_owned(),
                    "zoneArg".to_owned(),
                    DiInputTransformation::LocalToZonedDateTime))
        ));
        transformation_requests.push(TransformationRequest::new(
            TransformationDefinition::new(3,
                                          TransformationType::DoubleInput,
                                          "isAfterSunset".to_owned()),
            Transformation::Bi(
                DoubleInputTransformationDefinition::new(
                    3,
                    "zonedDateTimeForPosition".to_owned(),
                    "geoArg".to_owned(),
                    DiInputTransformation::IsAfterSunset))
        ));
        let tree_definition = SelectionTreeDefinition::new(1,
                                                           "test selection tree".to_owned());
        let evaluator = TestUtils::build_evaluator();
        let mut commands = Vec::new();
        commands.push(ContentCommand::HtmlCommand);
        ContentPipeline::new(1,
                             tenant_id.clone(),
                             ArgumentsExtractor::new(argument_definitions),
                             Transformer::new(transformation_requests),
                             SelectionTree::new(tenant_id.clone(),
                                                tree_definition,
                                                evaluator),
                             ContentCommandExecutor::new(tenant_id.clone(),
                                                         ContentType::Html,
                                                         commands)
        )
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
                    SimpleSelectionNodeDetails::new(0, 0),
                    ContentCommandAddress::new(0, 0)
                ));
        let nodes: Vec<SelectionNode> = vec![
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        1, "First After Condition Node".to_string()),
                    vec![SelectionEdgeAddress::new(2, 2)],
                    SimpleSelectionNodeDetails::new(1, 1),
                    ContentCommandAddress::new(1, 1)
                )),
            SelectionNode::Simple(
                SimpleSelectionNode::new(
                    SelectionNodeDefinition::new(
                        2, "Second Default Node".to_string()),
                    vec![SelectionEdgeAddress::new(3, 3)],
                    SimpleSelectionNodeDetails::new(2, 2),
                    ContentCommandAddress::new(2, 2)
                )),
            SelectionNode::Dictionary(
                DictionarySelectionNode::new(
                    SelectionNodeDefinition::new(
                        3, "Third Dictionary Node".to_string()),
                    vec![],
                    DictionarySelectionNodeDetails::new(
                        3, 3,
                        "dayOfWeekArg".to_string()),
                    DictionaryNodeMapping::new(
                        ContentCommandAddress::new(3, 3),
                        TestUtils::build_map(
                            vec!{
                                (ValueHolder::DayOfWeek(
                                    WeekdayWrapper::new(Weekday::Sat)),
                                 ContentCommandAddress::new(4, 4)),
                                (ValueHolder::DayOfWeek(
                                    WeekdayWrapper::new(Weekday::Sun)),
                                 ContentCommandAddress::new(5, 5))
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
                        "dayOfWeekArg".to_string()),
                    DictionaryNodeMapping::new(
                        ContentCommandAddress::new(6, 6),
                        TestUtils::build_map(
                            vec!{
                                (ValueHolder::DayOfWeek(
                                    WeekdayWrapper::new(Weekday::Sat)),
                                 ContentCommandAddress::new(7, 7)),
                                (ValueHolder::DayOfWeek(
                                    WeekdayWrapper::new(Weekday::Sun)),
                                 ContentCommandAddress::new(8, 8)),
                                (ValueHolder::DayOfWeek(
                                    WeekdayWrapper::new(Weekday::Mon)),
                                 ContentCommandAddress::new(9, 9))
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
                                               "dayOfWeekForDateTimeArg".to_owned(),
                                               RelationalOperator::Equals,
                                               false,
                                               ConditionValue::Runtime(
                                                   "dayOfWeekArg".to_owned())),
                                Condition::new(1,
                                               "decimalArg".to_owned(),
                                               RelationalOperator::LessThan,
                                               false,
                                               ConditionValue::Static(
                                                   ValueHolder::Decimal(
                                                       "11/2".parse::<BigRational>().unwrap())))
                            ],
                            Option::None)
                    ],
                    Expression::new(
                        ExpressionDefinition::new(
                            0,  LogicalOperator::And),
                        vec![
                            Condition::new(2,
                                           "dayOfWeekForDateTimeArg".to_owned(),
                                           RelationalOperator::Equals,
                                           false,
                                           ConditionValue::Runtime(
                                               "dayOfWeekArg".to_owned())),
                            Condition::new(3,
                                           "decimalArg".to_owned(),
                                           RelationalOperator::LessThan,
                                           false,
                                           ConditionValue::Static(
                                               ValueHolder::Decimal(
                                                   "11/2".parse::<BigRational>().unwrap()))),
                            Condition::new(4,
                                           "decimalArg".to_owned(),
                                           RelationalOperator::GreaterThanOrEquals,
                                           true,
                                           ConditionValue::Static(
                                               ValueHolder::Decimal(
                                                   "22/21".parse::<BigRational>().unwrap())))
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
                                           "isAfterSunset".to_owned(),
                                           RelationalOperator::Equals,
                                           false,
                                           ConditionValue::Static(
                                               ValueHolder::Boolean(true)
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
        SelectionTreeEvaluator::new(
            start_node,
            nodes,
            edges
        )
    }

    fn build_map<K, V>(entries: Vec<(K, V)>) -> HashMap<K, V>
        where K: Hash + Eq {
        let mut ret = HashMap::new();
        for entry in entries {
            ret.insert(entry.0, entry.1);
        }
        ret
    }

}