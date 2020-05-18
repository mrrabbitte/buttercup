use crate::app::content::commands::{ContentCommandsContext, ContentCommandExecutionError, ContentCommand};
use crate::app::values::ValuesPayload;
use crate::app::content::responses::ContentCommandResponse;

pub struct HtmlContentCommandsContext {

}

impl ContentCommandsContext for HtmlContentCommandsContext {

    fn execute(&self,
               payload: &ValuesPayload,
               content_commands: &Vec<ContentCommand>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        unimplemented!()
    }
}