use chrono::NaiveDateTime;
use uuid::Uuid;
use std::sync::Arc;
use crate::app::selection::tree::decision::{SelectionDecisionService, SelectionDecisionError, SelectionDecision};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReinforcementEvent {

    id: Uuid,
    tenant_id: String,
    created_at_utc: NaiveDateTime,
    decision_id: Uuid,
    event_type: ReinforcementEventType

}

#[derive(Debug)]
pub struct SimpleSuccessFailureReportRequest {

    tenant_id: String,
    pipeline_id: i32,
    command_ids: Vec<i32>

}

#[derive(Debug)]
pub struct SimpleSuccessFailureReport {

    report: Vec<SuccessFailureDetails>

}

impl SimpleSuccessFailureReport {

    pub fn get(&self) -> &Vec<SuccessFailureDetails> {
        &self.report
    }

}

#[derive(Debug)]
pub struct SuccessFailureDetails {

    command_id: i32,
    num_successes: i32,
    num_failures: i32

}

impl SuccessFailureDetails {

    pub fn get_command_id(&self) -> &i32 {
        &self.command_id
    }

    pub fn get_num_successes(&self) -> &i32 {
        &self.num_successes
    }

    pub fn get_num_failures(&self) -> &i32 {
        &self.num_failures
    }

}

#[derive(Debug)]
pub enum ReinforcementEventType {

    Success,
    Failure

}

pub enum ReinforcementServiceError {

    SelectionDecisionError(SelectionDecisionError)

}

pub struct ReinforcementService {

    decision_service: Arc<SelectionDecisionService>

}

impl ReinforcementService {

    pub fn handle(&self,
                  event: ReinforcementEvent) -> Result<(), ReinforcementServiceError> {
        match self.decision_service.get_decision_by_id(&event.decision_id) {
            Ok(decision) => {},
            Err(err) => Result::Err(err),
        }
    }

    pub fn get_simple_report(&self,
                             request: SimpleSuccessFailureReportRequest)
        -> Result<SimpleSuccessFailureReport, ReinforcementServiceError> {

    }

    fn do_handle(decision: SelectionDecision,
                 event: ReinforcementEvent) -> Result<(), ReinforcementServiceError> {
        
    }

}