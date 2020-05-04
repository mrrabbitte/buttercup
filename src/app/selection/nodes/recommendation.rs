use std::collections::HashMap;

use rand_distr::BetaError;

use crate::app::content::commands::ContentCommandAddress;
use crate::app::reinforcement::ReinforcementServiceError;
use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::{SelectionNodeDefinition, SelectionNodeDelegate, SelectionNodeError};
use crate::app::selection::nodes::context::SelectionNodesContext;
use crate::app::selection::nodes::recommendation::beta_bandits::{BetaBanditRecommender, BetaBanditResponse};
use crate::app::values::ValuesPayload;

pub mod beta_bandits;

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
    node_type: RecommendationSelectionNodeType,
    choice_space: Vec<ContentCommandAddress>

}

#[derive(Debug)]
pub enum RecommendationSelectionNodeType {

    SimpleBetaBandit

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationSelectionError {

    ReinforcementServiceError(ReinforcementServiceError),
    BetaError(BetaError),
    DidNotFindCommandOfRecommendedId(i32),
    DidNotFindCommandOfIndex(usize)

}

impl RecommendationSelectionNode {

    fn handle_recommendation_result(
        &self,
        result: Result<BetaBanditResponse, RecommendationSelectionError>)
        -> Result<&ContentCommandAddress, SelectionNodeError> {
        match result {
            Ok(response) => {
                let chosen_command_index = *response.get_chosen_command_index();
                match self.choice_space.get(chosen_command_index) {
                    None => Result::Err(
                        SelectionNodeError::RecommendationSelectionError(
                            RecommendationSelectionError::DidNotFindCommandOfIndex(
                                chosen_command_index))),
                    Some(address) => Result::Err(),
                }
            },
            Err(err) =>
                Result::Err(SelectionNodeError::RecommendationSelectionError(err)),
        }
    }

}

impl SelectionNodeDelegate for RecommendationSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress> {
        &self.outgoing_edges
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload,
                                 context: &dyn SelectionNodesContext)
                                 -> Result<&ContentCommandAddress, SelectionNodeError> {
        match self.node_type {
            RecommendationSelectionNodeType::SimpleBetaBandit =>
                self.handle_recommendation_result(
                    BetaBanditRecommender::choose_best_command(
                        &self.tenant_id,
                        &self.choice_space,
                        context
                    )),
        }
    }

}