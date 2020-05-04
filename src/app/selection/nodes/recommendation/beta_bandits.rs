use std::collections::HashMap;

use rand_distr::{Beta, Distribution};

use crate::app::content::commands::ContentCommandAddress;
use crate::app::reinforcement::{ReinforcementServiceError, SimpleSuccessFailureReport};
use crate::app::selection::nodes::context::SelectionNodesContext;
use crate::app::selection::nodes::recommendation::RecommendationSelectionError;
use crate::app::selection::nodes::SelectionNodeError;
use crate::app::values::ValuesPayload;

pub struct BetaBanditRecommender;

impl BetaBanditRecommender {

    pub fn choose_best_command(tenant_id: &String,
                               choice_space: &Vec<ContentCommandAddress>,
                               context: &dyn SelectionNodesContext)
                               -> Result<BetaBanditResponse, RecommendationSelectionError> {
        match context.get_success_failures_report(tenant_id, choice_space) {
            Ok(report) =>
                BetaBanditRecommender::handle_report(&report),
            Err(err) => Result::Err(
                RecommendationSelectionError::ReinforcementServiceError(err)),
        }
    }

    fn handle_report(report: &SimpleSuccessFailureReport)
                     -> Result<BetaBanditResponse, RecommendationSelectionError> {
        let mut highest_score = -1.;
        let mut highest_score_command_id = -1;
        let mut highest_score_command_index = -1;
        let all_details = report.get();
        for i in 0..all_details.len() {
            let details = &all_details[i];
            match Beta::new(*details.get_num_successes(), *details.get_num_failures()) {
                Ok(beta) => {
                    if beta.sample(&mut rand::thread_rng()) > highest_score {
                        highest_score_command_id = *details.get_command_id();
                        highest_score_command_index = i;
                    }
                },
                Err(err) =>
                    return Result::Err(RecommendationSelectionError::BetaError(err))
            }
        }

        Result::Ok(
            BetaBanditResponse::new(
                highest_score_command_index,
                highest_score_command_id))
    }

}

pub struct BetaBanditResponse {

    chosen_command_index: usize,
    chosen_command_id: i32

}

impl BetaBanditResponse {

    pub fn new(chosen_command_index: usize,
               chosen_command_id: i32) -> BetaBanditResponse {
        BetaBanditResponse {
            chosen_command_index,
            chosen_command_id
        }
    }

    pub fn get_chosen_command_index(&self) -> &usize {
        &self.chosen_command_index
    }

    pub fn get_chosen_command_id(&self) -> &i32 {
        &self.chosen_command_id
    }

}