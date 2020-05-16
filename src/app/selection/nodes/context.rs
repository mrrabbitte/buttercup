use crate::app::reinforcement::{SimpleSuccessFailureReport, ReinforcementServiceError, ReinforcementService};
use mockall::automock;
use std::collections::HashMap;
use crate::app::content::commands::ContentCommandAddress;

#[automock]
pub trait SelectionNodesContext {

    fn get_success_failures_report(&self,
                                   tenant_id: &String,
                                   choice_space: &Vec<ContentCommandAddress>)
        -> Result<SimpleSuccessFailureReport, ReinforcementServiceError>;

}

#[derive(Clone)]
pub struct SimpleSelectionNodesContext {

    reinforcement_service: ReinforcementService

}

impl SimpleSelectionNodesContext {

    pub fn new(reinforcement_service: ReinforcementService) -> SimpleSelectionNodesContext {
        SimpleSelectionNodesContext {
            reinforcement_service
        }
    }

}

impl SelectionNodesContext for SimpleSelectionNodesContext {

    fn get_success_failures_report(&self,
                                   tenant_id: &String,
                                   choice_space:  &Vec<ContentCommandAddress>)
        -> Result<SimpleSuccessFailureReport, ReinforcementServiceError> {
        self.reinforcement_service.get_simple_report(tenant_id, choice_space)
    }

}

