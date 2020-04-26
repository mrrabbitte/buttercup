use crate::app::content::commands::ContentCommandAddress;
use uuid::Uuid;
use dashmap::DashMap;
use std::sync::Arc;
use dashmap::mapref::one::Ref;
use crate::app::common::addressable::Address;

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
            content_commands.iter().map(|addr| addr.get_id()).collect()
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

pub enum SelectionDecisionError {

    CouldNotSaveDecision,
    CouldNotFindDecisionById

}

pub struct SelectionDecisionService {

    mock_decision_repo: Arc<DashMap<Uuid, SelectionDecision>>

}

impl SelectionDecisionService {

    pub fn save(&self,
                decision: &SelectionDecision) -> Result <(), SelectionDecisionError> {
        self.mock_decision_repo.insert(decision.id, decision.clone());
        Result::Ok(())
    }

    pub fn get_decision_by_id(&self,
                              decision_id: &Uuid)
                              -> Result<&SelectionDecision, SelectionDecisionError> {
        match self.mock_decision_repo.get(decision_id) {
            None => Result::Err(SelectionDecisionError::CouldNotFindDecisionById),
            Some(decision) => Result::Ok(decision.value()),
        }
    }

}