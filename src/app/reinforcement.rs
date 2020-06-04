use std::sync::Arc;

use chrono::NaiveDateTime;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::decision::{SelectionDecision, SelectionDecisionError, SelectionDecisionService};
use crate::app::content::commands::ContentCommandAddress;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReinforcementEvent {

    id: Uuid,
    tenant_id: String,
    created_at_utc: NaiveDateTime,
    decision_id: Uuid,
    event_type: ReinforcementEventType

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReinforcementEventType {

    Success,
    Failure

}

#[derive(Debug, Clone)]
pub struct SimpleSuccessFailureReport {

    report: Vec<SuccessFailureDetails>

}

impl SimpleSuccessFailureReport {

    pub fn new(report: Vec<SuccessFailureDetails>) -> SimpleSuccessFailureReport {
        SimpleSuccessFailureReport {
            report
        }
    }

    pub fn get(&self) -> &Vec<SuccessFailureDetails> {
        &self.report
    }

}

#[derive(Debug, Clone)]
pub struct SuccessFailureDetails {

    command_id: i32,
    num_successes: u32,
    num_failures: u32

}

impl SuccessFailureDetails {

    pub fn new(command_id: i32,
               num_successes: u32,
               num_failures: u32) -> SuccessFailureDetails {
        SuccessFailureDetails {
            command_id,
            num_successes,
            num_failures
        }
    }

    pub fn get_command_id(&self) -> &i32 {
        &self.command_id
    }

    pub fn get_num_successes(&self) -> &u32 {
        &self.num_successes
    }

    pub fn get_num_failures(&self) -> &u32 {
        &self.num_failures
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReinforcementServiceError {

    SelectionDecisionError(SelectionDecisionError),
    SuccessFailureReportError

}

#[derive(Clone)]
pub struct ReinforcementService {

    decision_service: SelectionDecisionService

}

impl ReinforcementService {

    pub fn new(decision_service: SelectionDecisionService) -> ReinforcementService {
        ReinforcementService {
            decision_service
        }
    }

    pub fn handle(&self,
                  event: &ReinforcementEvent) -> Result<(), ReinforcementServiceError> {
        match self.decision_service.get_decision_by_id(&event.decision_id) {
            Ok(decision) => self.do_handle(decision, event),
            Err(err) => Result::Err(
                ReinforcementServiceError::SelectionDecisionError(err)),
        }
    }

    pub fn get_simple_report(&self,
                             tenant_id: &String,
                             choice_space: &Vec<ContentCommandAddress>)
        -> Result<SimpleSuccessFailureReport, ReinforcementServiceError> {
        unimplemented!()
    }

    fn do_handle(&self,
                 decision: &SelectionDecision,
                 event: &ReinforcementEvent) -> Result<(), ReinforcementServiceError> {
        unimplemented!()
    }

}
