use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutionError, ContentCommandsContext, ContentCommandExecutorDelegate, ContentCommandExecutorContexts};
use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::ValuesPayload;
use crate::app::content::definitions::ContentType;

pub mod append;
pub mod sources;

#[derive(Debug, Clone)]
pub struct VideoContentCommandsContext {

}

impl VideoContentCommandsContext {

    pub fn new() -> VideoContentCommandsContext {
        VideoContentCommandsContext{}
    }

}

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
