use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::app::content::commands::ContentCommandExecutionError;
use crate::app::content::commands::html::HtmlContentCommandError;
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod builder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendHtmlFromTemplateCommand {

    id: i32,
    operations: Vec<TemplateOperation>

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateOperation {

    AddContent(String),
    AddValue(String)

}

impl AppendHtmlFromTemplateCommand {

    pub fn new(id: i32,
               operations: Vec<TemplateOperation>) -> AppendHtmlFromTemplateCommand {
        AppendHtmlFromTemplateCommand {
            id,
            operations
        }
    }

    pub fn execute(&self,
                   payload: &ValuesPayload,
                   target: &mut dyn Write) -> Result<(), ContentCommandExecutionError> {
        for operation in &self.operations {
            match operation {
                TemplateOperation::AddContent(content) => {
                    target.write(content.as_bytes());
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr};

    use crate::app::content::commands::html::template::builder::AppendHtmlFromTemplateCommandBuilder;
    use crate::app::values::email::Email;

    use super::*;

    #[test]
    fn test_writes_template_with_provided_values() {
        let template = "\
        Hello, {{name}}! \
        \n Your email: {{email}}. \
        \n How are you doing? \
        I can see that you use the following ip: {{ip}}.\
        \n Some other content that is not a tag."
            .to_owned();
        let command =
            AppendHtmlFromTemplateCommandBuilder::build(0, template);
        let mut values = HashMap::new();
        values.insert("name".to_owned(), ValueHolder::String("SomeName".to_owned()));
        values.insert("email".to_owned(), ValueHolder::Email(
            Email::new("some@example.com").unwrap()));
        values.insert("ip".to_owned(), ValueHolder::IpAddress(
            IpAddr::V4(
                Ipv4Addr::new(192, 168, 0, 1))));
        let payload = ValuesPayload::new(values);
        let mut output: Vec<u8> = Vec::new();
        let result = command.execute(&payload, &mut output);
        assert_eq!(true, result.is_ok());
        assert_eq!("\
        Hello, SomeName! \
        \n Your email: some@example.com. \
        \n How are you doing? \
        I can see that you use the following ip: 192.168.0.1.\
        \n Some other content that is not a tag."
            .to_owned(),
                   String::from_utf8(output).unwrap());
    }

}
