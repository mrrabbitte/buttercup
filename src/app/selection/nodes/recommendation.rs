use crate::app::selection::nodes::{SelectionNodeDefinition, SelectionNodeDelegate};
use crate::app::values::ValuesPayload;

pub struct RecommendationSelectionNodeDetails {

    selection_node_definition_id: i32,
    mock_best_content_command_definition_id: i32

}

pub struct RecommendationSelectionNode<'a> {

    service: &'a RecommendationService,
    tenant_id: String,
    definition: SelectionNodeDefinition,
    outgoing_edge_ids: Vec<i32>,
    details: RecommendationSelectionNodeDetails

}

pub struct RecommendationService;

impl SelectionNodeDelegate for RecommendationSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edge_ids(&self) -> &Vec<i32> {
        &self.outgoing_edge_ids
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload) -> &i32 {
        unimplemented!()
    }

}