use std::collections::HashMap;

use crate::app::arguments::ArgumentDefinition;
use crate::app::selection::edges::{SelectionEdge, SelectionEdgeDelegate};
use crate::app::selection::nodes::SelectionNode;
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
    selection_nodes: Vec<SelectionNode>,
    selection_edges: Vec<SelectionEdge>

}