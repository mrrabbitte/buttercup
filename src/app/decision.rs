use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::common::addressable::Address;
use crate::app::content::commands::ContentCommandAddress;

#[derive(Debug, Clone)]
pub struct SelectionDecision {

    id: Uuid,
    pipeline_id: i32,
    content_commands: Vec<ContentCommandAddress>

}

impl SelectionDecision {

    pub fn new(pipeline_id: i32,
               content_commands: Vec<ContentCommandAddress>) -> SelectionDecision {
        SelectionDecision {
            id: Uuid::new_v4(),
            pipeline_id,
            content_commands
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_pipeline_id(&self) -> &i32 {
        &self.pipeline_id
    }

    pub fn get_content_commands(&self) -> &Vec<ContentCommandAddress> {
        &self.content_commands
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectionDecisionError {

    CouldNotSaveDecision,
    CouldNotFindDecisionById

}

#[derive(Clone)]
pub struct SelectionDecisionService {

}

impl SelectionDecisionService {

    pub fn new() -> SelectionDecisionService {
        SelectionDecisionService{}
    }

    pub fn save(&self,
                decision: &SelectionDecision) -> Result <(), SelectionDecisionError> {
        unimplemented!()
    }

    pub fn get_decision_by_id(&self,
                              decision_id: &Uuid)
                              -> Result<&SelectionDecision, SelectionDecisionError> {
        unimplemented!()
    }

}