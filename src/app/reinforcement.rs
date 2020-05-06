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
    num_successes: f32,
    num_failures: f32

}

impl SuccessFailureDetails {

    pub fn get_command_id(&self) -> &i32 {
        &self.command_id
    }

    pub fn get_num_successes(&self) -> &f32 {
        &self.num_successes
    }

    pub fn get_num_failures(&self) -> &f32 {
        &self.num_failures
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReinforcementServiceError {

    SelectionDecisionError(SelectionDecisionError)

}

pub struct ReinforcementService {

    decision_service: Arc<SelectionDecisionService>

}

impl ReinforcementService {

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


pub struct ContentCommandSuccessEvent {

    id: String,
    created_at_utc: NaiveDateTime,
    content_command_definition_id: i32,
    selection_decision_id: i32

}

pub struct ContentCommandFailureEvent {

}