use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::app::content::commands::ContentCommandExecutionError;
use crate::app::content::commands::html::HtmlContentCommandError;
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod builder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendHtmlFromTemplateCommand {

    id: i32,
    operations: Vec<TemplateOperation>,
    template: Vec<u8>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateOperation {

    AddContent(usize, usize),
    AddValue(String)

}

impl AppendHtmlFromTemplateCommand {

    pub fn new(id: i32,
               operations: Vec<TemplateOperation>,
               template: Vec<u8>) -> AppendHtmlFromTemplateCommand {
        AppendHtmlFromTemplateCommand {
            id,
            operations,
            template
        }
    }

    pub fn execute(&self,
                   payload: &ValuesPayload,
                   target: &mut dyn Write) -> Result<(), ContentCommandExecutionError> {
        for operation in &self.operations {
            match operation {
                TemplateOperation::AddContent(start_idx, end_idx) => {
                    target.write(&self.template[*start_idx..*end_idx]);
                },
                TemplateOperation::AddValue(value_name) => {
                    match payload.get(&value_name) {
                        None => return Result::Err(
                            ContentCommandExecutionError::HtmlContentCommandError(
                                HtmlContentCommandError::DidNotFindValue(value_name.clone()))),
                        Some(value) =>
                            match AppendHtmlFromTemplateCommand::handle_value(
                                &value_name, value, target) {
                                Ok(_) => {},
                                Err(err) =>
                                    return Result::Err(
                                        ContentCommandExecutionError::HtmlContentCommandError(err)),
                            },
                    }
                },
            }
        }
        Result::Ok(())
    }

    fn handle_value(name: &String,
                    value: &ValueHolder,
                    target: &mut dyn Write) -> Result<(), HtmlContentCommandError> {
        match value {
            ValueHolder::String(val) => target.write(val.as_bytes()),
            ValueHolder::Email(val) => target.write(val.get().as_bytes()),
            ValueHolder::IpAddress(val) => target.write(val.to_string().as_bytes()),
            _ => return Result::Err(
                HtmlContentCommandError::AmbiguousStringConversion(name.clone(), value.clone()))
        };
        Result::Ok(())
    }

}
