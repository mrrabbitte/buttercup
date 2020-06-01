use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::ValuesPayload;
use crate::app::content::definitions::ContentType;

use serde::{Serialize, Deserialize};
use crate::app::content::commands::{ContentCommandExecutorDelegate, ContentCommandExecutorContexts, ContentCommandAddress, ContentCommandExecutionError};

pub mod append;
pub mod sources;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoContentCommandsContext {

}

impl VideoContentCommandsContext {

    pub fn new() -> VideoContentCommandsContext {
        VideoContentCommandsContext{}
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoContentCommandsExecutor;

impl ContentCommandExecutorDelegate for VideoContentCommandsExecutor {

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        unimplemented!()
    }

    fn get_content_type(&self) -> ContentType {
        ContentType::Video
    }
}
