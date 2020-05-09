use std::collections::HashMap;

use rand_distr::{Beta, Distribution};

use crate::app::content::commands::ContentCommandAddress;
use crate::app::reinforcement::{ReinforcementServiceError, SimpleSuccessFailureReport};
use crate::app::selection::nodes::context::SelectionNodesContext;
use crate::app::selection::nodes::recommendation::RecommendationSelectionError;
use crate::app::selection::nodes::SelectionNodeError;
use crate::app::values::ValuesPayload;
use crate::app::selection::nodes::recommendation::response::RecommenderResponse;

pub struct BetaBanditRecommender;

impl BetaBanditRecommender {

    pub fn choose_best_command(tenant_id: &String,
                               choice_space: &Vec<ContentCommandAddress>,
                               context: &dyn SelectionNodesContext)
                               -> Result<RecommenderResponse, RecommendationSelectionError> {
        match context.get_success_failures_report(tenant_id, choice_space) {
            Ok(report) =>
                BetaBanditRecommender::handle_report(&report),
            Err(err) => Result::Err(
                RecommendationSelectionError::ReinforcementServiceError(err)),
        }
    }

    fn handle_report(report: &SimpleSuccessFailureReport)
                     -> Result<RecommenderResponse, RecommendationSelectionError> {
        let mut highest_score = -1.;
        let mut highest_score_command_id = -1;
        let mut highest_score_command_index = 0;
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
            RecommenderResponse::new(
                highest_score_command_index,
                highest_score_command_id))
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    const TENANT_ID: &str = "tenant_1";

    #[test]
    fn test_variant_is_chosen_out_of_choice_space() {
        let context = MockSelectionNodesContext::new();
        context.expect_get_success_failures_report()
    }

    #[test]
    fn test_variant_with_best_success_failure_report_is_chosen() {

    }

    #[test]
    fn test_error_from_context_is_forwarded() {

    }

    fn build_choice_space() -> Vec<ContentCommandAddress> {
        
    }

}
