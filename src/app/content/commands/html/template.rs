use crate::app::content::commands::{ContentCommand, ContentCommandExecutionError};
use std::io::Write;
use crate::app::values::ValuesPayload;
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
                    target.write(self.blocks.get(current_idx_blocks).unwrap());
                    current_idx_blocks += 1;
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