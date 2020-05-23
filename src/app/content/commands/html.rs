use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutionError, ContentCommandsContext};
use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::ValuesPayload;

pub mod template;

#[derive(Debug, Clone)]
pub struct HtmlContentCommandsContext {

}

impl HtmlContentCommandsContext {

    pub fn new() -> HtmlContentCommandsContext {
        HtmlContentCommandsContext{}
    }

}

impl ContentCommandsContext for HtmlContentCommandsContext {

    fn execute(&self,
               payload: &ValuesPayload,
               content_commands: &Vec<ContentCommand>,
               addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        Result::Err(ContentCommandExecutionError::NoCommandsProvided)
    }
}