use crate::app::content::commands::{ContentCommandsContext, ContentCommand, ContentCommandExecutionError};
use crate::app::values::ValuesPayload;
use crate::app::content::responses::ContentCommandResponse;

pub mod append;
pub mod sources;

pub struct VideoContentCommandsContext {

}

impl ContentCommandsContext for VideoContentCommandsContext {

    fn execute(&self,
               payload: &ValuesPayload,
               content_commands: &Vec<ContentCommand>)
               -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        unimplemented!()
    }
}