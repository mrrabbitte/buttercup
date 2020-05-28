use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutionError, ContentCommandsContext, ContentCommandDelegate, ContentCommandExecutorDelegate, ContentCommandExecutorContexts};
use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::ValuesPayload;
use crate::app::files::FileService;
use std::io::Write;
use crate::app::common::addressable::Address;
use crate::app::content::definitions::ContentType;
use serde::{Serialize, Deserialize};

pub mod template;

#[derive(Serialize, Deserialize)]
pub enum HtmlContentCommand {

    TemplateCommand

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HtmlContentCommandError {

    DidNotFindRequestedBlock(usize),
    DidNotFindValueName(String)

}

#[derive(Debug, Clone)]
pub struct HtmlContentCommandsContext {

    file_service: FileService

}

#[derive(Serialize, Deserialize)]
pub struct HtmlContentCommandExecutor {

    tenant_id: String,
    commands: Vec<HtmlContentCommand>

}

impl ContentCommandExecutorDelegate for HtmlContentCommandExecutor {

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        let context = contexts.get_html_context();
    }

    fn get_content_type(&self) -> ContentType {
        ContentType::Html
    }

}