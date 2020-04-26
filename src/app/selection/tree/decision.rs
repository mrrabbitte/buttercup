use crate::app::content::commands::ContentCommandAddress;
use uuid::Uuid;

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

pub enum SelectionDecisionError {

}

pub struct SelectionDecisionService {

}

impl SelectionDecisionService {

    pub fn handle(&self,
                  decision: &SelectionDecision) -> Result <(), SelectionDecisionError> {
        unimplemented!()
    }

}