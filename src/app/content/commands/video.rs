use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutionError, ContentCommandsContext};
use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::ValuesPayload;

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

impl ContentCommandsContext for VideoContentCommandsContext {

    fn execute(&self,
               payload: &ValuesPayload,
               content_commands: &Vec<ContentCommand>,
               addresses: &Vec<ContentCommandAddress>)
               -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        unimplemented!()
    }
}