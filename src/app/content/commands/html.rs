use crate::app::content::commands::{ContentCommand, ContentCommandAddress, ContentCommandExecutionError, ContentCommandDelegate, ContentCommandExecutorDelegate, ContentCommandExecutorContexts};
use crate::app::content::responses::ContentCommandResponse;
use crate::app::values::{ValuesPayload, ValueHolder};
use crate::app::files::{FileService, FilesServiceError, FileResponse};
use std::io::Write;
use crate::app::common::addressable::Address;
use crate::app::content::definitions::ContentType;
use serde::{Serialize, Deserialize};
use crate::app::content::commands::html::template::AppendHtmlFromTemplateCommand;
use std::fs::File;

pub mod template;

#[derive(Serialize, Deserialize)]
pub enum HtmlContentCommand {

    AppendHtmlFromTemplateCommand(AppendHtmlFromTemplateCommand)

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HtmlContentCommandError {

    DidNotFindValue(String),
    AmbiguousStringConversion(String, ValueHolder)

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

impl HtmlContentCommandExecutor {

    fn execute_commands(&self,
                        target: FileResponse,
                        payload: &ValuesPayload,
                        addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        let mut file = target.get_file();
        for address in addresses {
            match self.commands.get(address.index) {
                None => return Result::Err(
                    ContentCommandExecutionError::ContentCommandNotFound(address.clone())),
                Some(html_command) => {
                    match html_command {
                        HtmlContentCommand::AppendHtmlFromTemplateCommand(
                            command) => {
                            match command.execute(payload, file) {
                                Ok(_) => {},
                                Err(err) =>
                                    return Result::Err(err),
                            }
                        },
                    }
                }
            }
        }
        file.flush();
        return Result::Ok(ContentCommandResponse::new(target.get_external_path().clone()))
    }

}

impl ContentCommandExecutorDelegate for HtmlContentCommandExecutor {

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        match contexts.get_html_context().file_service.create_new_html(&self.tenant_id) {
            Ok(target) => self.execute_commands(target, payload, addresses),
            Err(err) =>
                Result::Err(ContentCommandExecutionError::FilesServiceError(err)),
        }
    }

    fn get_content_type(&self) -> ContentType {
        ContentType::Html
    }

}