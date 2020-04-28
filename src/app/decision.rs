use crate::app::content::commands::ContentCommandAddress;
use uuid::Uuid;
use dashmap::DashMap;
use std::sync::Arc;
use dashmap::mapref::one::Ref;
use crate::app::common::addressable::Address;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct SelectionDecision {

    id: Uuid,
    pipeline_id: i32,
    content_command_ids: Vec<i32>

}

impl SelectionDecision {

    pub fn new(pipeline_id: i32,
               content_commands: Vec<ContentCommandAddress>) -> SelectionDecision {
        SelectionDecision {
            id: Uuid::new_v4(),
            pipeline_id,
            content_command_ids:
            content_commands
                .iter()
                .map(|addr| *addr.get_id())
                .collect::<Vec<i32>>()
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_pipeline_id(&self) -> &i32 {
        &self.pipeline_id
    }

    pub fn get_content_commands(&self) -> &Vec<i32> {
        &self.content_command_ids
    }

}

pub enum SelectionDecisionError {

    CouldNotSaveDecision,
    CouldNotFindDecisionById

}

pub struct SelectionDecisionService {

}

impl SelectionDecisionService {

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