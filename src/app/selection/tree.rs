use std::collections::HashMap;

use crate::app::arguments::ArgumentDefinition;
use crate::app::selection::nodes::{SelectionNode, SelectionNodeDelegate};
use crate::app::transformations::transformer::TransformationRequest;

pub struct SelectionTreeDefinition {

    id: i32,
    name: String,
    project_definition_id: i32

}

pub struct SelectionTree {

    tenant_id: String,
    argument_definitions: HashMap<String, ArgumentDefinition>,
    transformation_requests: Vec<TransformationRequest>,
    selection_nodes: HashMap<i32, SelectionNode>

}