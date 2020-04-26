use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::{SelectionNodeError, SelectionNodeDefinition, SelectionNodeDelegate};
use crate::app::values::ValuesPayload;
use crate::app::content::commands::ContentCommandAddress;
use std::collections::HashMap;
use crate::app::selection::nodes::context::SelectionNodesContext;

#[derive(Debug)]
pub struct RecommendationSelectionNodeDetails {

    id: i32,
    selection_node_definition_id: i32,

}


#[derive(Debug)]
pub struct RecommendationSelectionNode {

    tenant_id: String,
    definition: SelectionNodeDefinition,
    outgoing_edges: Vec<SelectionEdgeAddress>,
    details: RecommendationSelectionNodeDetails,
    choose_from_commands: HashMap<i32, ContentCommandAddress>

}

pub struct RecommendationService;

impl SelectionNodeDelegate for RecommendationSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress> {
        &self.outgoing_edges
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload,
                                 context: &SelectionNodesContext)
        -> Result<&ContentCommandAddress, SelectionNodeError> {
        unimplemented!()
    }

}