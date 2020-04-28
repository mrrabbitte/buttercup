use crate::app::reinforcement::{SimpleSuccessFailureReportRequest, SimpleSuccessFailureReport, ReinforcementServiceError};
use mockall::automock;

#[automock]
pub trait SelectionNodesContext {

    fn get_success_failures_report(&self,
                                   request: SimpleSuccessFailureReportRequest)
        -> Result<SimpleSuccessFailureReport, ReinforcementServiceError>;

}

pub struct SimpleSelectionNodesContext {



}

