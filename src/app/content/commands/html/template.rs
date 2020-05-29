use crate::app::content::commands::{ContentCommand, ContentCommandExecutionError};
use std::io::Write;
use crate::app::values::{ValuesPayload, ValueHolder};
use crate::app::content::commands::html::HtmlContentCommandError;

pub struct AppendHtmlFromTemplateCommand {

    id: i32,
    operations: Vec<TemplateOperation>,
    blocks: Vec<[u8]>,
    value_names: Vec<String>

}

enum TemplateOperation {

    AddBlock,
    AddValue

}

impl AppendHtmlFromTemplateCommand {

    pub fn new(id: i32,
               operations: Vec<TemplateOperation>,
               blocks: Vec<[u8]>,
               value_names: Vec<String>) -> AppendHtmlFromTemplateCommand {
        AppendHtmlFromTemplateCommand {
            id,
            operations,
            blocks,
            value_names
        }
    }

    pub fn execute(&self,
                   payload: &ValuesPayload,
                   target: &mut dyn Write) -> Result<(), ContentCommandExecutionError> {
        let mut current_idx_blocks = 0;
        let mut current_idx_value_names = 0;
        for operation in self.operations {
            match operation {
                TemplateOperation::AddBlock => {
                    match self.blocks.get(current_idx_blocks) {
                        None =>
                            return Result::Err(
                                ContentCommandExecutionError::HtmlContentCommandError(
                                    HtmlContentCommandError::DidNotFindRequestedBlock(
                                        current_idx_blocks))),
                        Some(block) => {
                            target.write(block);
                            current_idx_blocks += 1;
                        },
                    }
                },
                TemplateOperation::AddValue => {
                    match self.value_names.get(current_idx_value_names) {
                        None => return Result::Err(
                            ContentCommandExecutionError::HtmlContentCommandError(
                                HtmlContentCommandError::DidNotFindRequestedValueName(
                                    current_idx_value_names))),
                        Some(value_name) => {
                            match payload.get(value_name) {
                                None => return Result::Err(
                                    ContentCommandExecutionError::HtmlContentCommandError(
                                        HtmlContentCommandError::DidNotFindValue(
                                            value_name.clone()))),
                                Some(value) => {
                                    match ValuesToString::convert(value) {
                                        Ok(string_value) => {
                                            target.write(string_value.as_bytes());
                                            current_idx_value_names += 1;
                                        },
                                        Err(_) => return Result::Err(
                                            ContentCommandExecutionError::HtmlContentCommandError(
                                                HtmlContentCommandError::AmbigousStringValueConversion(
                                                    value_name.clone(), value.clone()))),
                                    }
                                },
                            }
                        },
                    }
                },
            }
        }
        Result::Ok(())
    }

}

impl ContentCommand for AppendHtmlFromTemplateCommand {

    fn get_id(&self) -> &i32 {
        &self.id
    }

}

struct ValuesToString;

impl ValuesToString {

    fn convert(value: &ValueHolder) -> Result<String, ()> {
        match value {
            ValueHolder::String(val) => Result::Ok(val.clone()),
            ValueHolder::Email(val) => Result::Ok(val.get().clone()),
            ValueHolder::IpAddress(val) => Result::Ok(val.to_string()),
            _ => Result::Err(())
        }
    }

}