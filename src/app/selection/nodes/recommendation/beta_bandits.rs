use std::collections::HashMap;

use rand_distr::{Beta, Distribution};

use crate::app::content::commands::ContentCommandAddress;
use crate::app::reinforcement::{ReinforcementServiceError, SimpleSuccessFailureReport};
use crate::app::selection::nodes::context::SelectionNodesContext;
use crate::app::selection::nodes::recommendation::RecommendationSelectionError;
use crate::app::selection::nodes::recommendation::response::RecommenderResponse;
use crate::app::selection::nodes::SelectionNodeError;
use crate::app::values::ValuesPayload;

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
            match Beta::new(
                BetaBanditRecommender::safe_for_beta(*details.get_num_successes()),
                BetaBanditRecommender::safe_for_beta(*details.get_num_failures())) {
                Ok(beta) => {
                    let score = beta.sample(&mut rand::thread_rng());
                    if score > highest_score {
                        highest_score_command_id = *details.get_command_id();
                        highest_score_command_index = i;
                        highest_score = score;
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

    fn safe_for_beta(value: u32) -> f32 {
        if 0 == value {
            return 1.0;
        }
        return value as f32;
    }

}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::app::common::addressable::Address;
    use crate::app::selection::nodes::context::MockSelectionNodesContext;

    use super::*;
    use crate::app::reinforcement::SuccessFailureDetails;
    use rand_distr::BetaError;

    const TENANT_ID: &str = "tenant_1";
    const CONTENT_COMMAND_ID: i32 = 1;
    const CONTENT_COMMAND_INDEX: usize = 0;

    #[test]
    fn test_variant_is_chosen_out_of_choice_space() {
        let choice_space  =
            vec![ContentCommandAddress::new(CONTENT_COMMAND_ID, CONTENT_COMMAND_INDEX)];
        let tenant_id = String::from(TENANT_ID);
        let mut context = build_mock_context(Result::Ok(SimpleSuccessFailureReport::new(
            vec![
                SuccessFailureDetails::new(
                    CONTENT_COMMAND_ID, 1000, 1)])));
        let result =
            BetaBanditRecommender::choose_best_command(&tenant_id, &choice_space, &context);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(*response.get_chosen_command_id(), CONTENT_COMMAND_ID);
        assert_eq!(*response.get_chosen_command_index(), CONTENT_COMMAND_INDEX);
    }

    #[test]
    fn test_variant_with_best_success_failure_ratio_is_chosen() {
        let choice_space  =
            vec![ContentCommandAddress::new(1, 0),
                 ContentCommandAddress::new(2, 1),
                 ContentCommandAddress::new(3, 2),
                 ContentCommandAddress::new(16, 3)];
        let tenant_id = String::from(TENANT_ID);
        let mut context = build_mock_context(Result::Ok(
            SimpleSuccessFailureReport::new(
                vec![
                    SuccessFailureDetails::new(
                        1, 1, 1),
                    SuccessFailureDetails::new(
                        2, 100000000, 1),
                    SuccessFailureDetails::new(
                        3, 1, 1),
                    SuccessFailureDetails::new(
                        16, 1, 1)
                ]
            )));
        let result =
            BetaBanditRecommender::choose_best_command(&tenant_id, &choice_space, &context);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(*response.get_chosen_command_id(), 2);
        assert_eq!(*response.get_chosen_command_index(), 1);
    }

    #[test]
    fn test_works_for_zero_counts() {
        let choice_space  =
            vec![ContentCommandAddress::new(CONTENT_COMMAND_ID, CONTENT_COMMAND_INDEX)];
        let tenant_id = String::from(TENANT_ID);
        let mut context = build_mock_context(Result::Ok(SimpleSuccessFailureReport::new(
            vec![
                SuccessFailureDetails::new(
                    CONTENT_COMMAND_ID, 0, 1)])));
        let result =
            BetaBanditRecommender::choose_best_command(&tenant_id, &choice_space, &context);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(*response.get_chosen_command_id(), CONTENT_COMMAND_ID);
        assert_eq!(*response.get_chosen_command_index(), CONTENT_COMMAND_INDEX);
    }

    #[test]
    fn test_error_from_context_is_forwarded() {
        let choice_space  =
            vec![ContentCommandAddress::new(CONTENT_COMMAND_ID, CONTENT_COMMAND_INDEX)];
        let tenant_id = String::from(TENANT_ID);
        let mut context = build_mock_context(Result::Err(
            ReinforcementServiceError::SuccessFailureReportError));
        let result =
            BetaBanditRecommender::choose_best_command(&tenant_id, &choice_space, &context);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(),
                   RecommendationSelectionError::ReinforcementServiceError(
                       ReinforcementServiceError::SuccessFailureReportError));
    }

    fn build_mock_context(report: Result<SimpleSuccessFailureReport, ReinforcementServiceError>)
                          -> MockSelectionNodesContext {
        let mut context = MockSelectionNodesContext::new();
        context
            .expect_get_success_failures_report()
            .times(1)
            .return_once(move |_, _| report);
        return context;
    }

}
