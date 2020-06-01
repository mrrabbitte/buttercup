use regex::{Error, Regex};

use crate::app::content::commands::html::template::{AppendHtmlFromTemplateCommand, TemplateOperation};

lazy_static! {
    static ref TAG_REGEX: Regex = Regex::new(r"\{\{.*?}}").unwrap();
}

pub(crate) struct AppendHtmlFromTemplateCommandBuilder;

impl AppendHtmlFromTemplateCommandBuilder {

    pub fn build(id: i32,
                 template: String) -> AppendHtmlFromTemplateCommand {
        let mut operations = Vec::new();
        let mut current_content_start_idx = 0;
        for m in TAG_REGEX.find_iter(&template) {
            let start = m.start();
            let end = m.end();
            if start - current_content_start_idx > 0 {
                operations.push(TemplateOperation::AddContent(
                    (&template[current_content_start_idx..start]).to_owned()))
            }
            let value_name = m.as_str().replace("{{", "").replace("}}", "");
            operations.push(TemplateOperation::AddValue(value_name));
            current_content_start_idx = end;
        }
        let template_len = template.len();
        match template_len.checked_sub(current_content_start_idx + 1) {
            Some(_) => operations.push(TemplateOperation::AddContent(
                (&template[current_content_start_idx..template_len]).to_owned())),
            None => {}
        };
        AppendHtmlFromTemplateCommand::new(id, operations)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converts_template_with_no_tailing_chars_to_operations_correctly() {
        let template = "Hello, {{firstName}}! \
        \n My name is {{yourName1}}. \
        \n How are you doing? \
        I'm {{yourMood}}. {{yourName2}},{{yourName3}}";
        let command =
            AppendHtmlFromTemplateCommandBuilder::build(0, template.to_owned());
        //assert_eq!(10, command.operations.len());
        let hello = "Hello, ".to_owned();
        let comma = ",".to_owned();
        let how_are_you = ". \
        \n How are you doing? \
        I'm ".to_owned();
        let your_name_3 = "yourName3".to_owned();
        assert!(matches!(command.operations.get(0),
            Some(TemplateOperation::AddContent(hello))));
        assert!(matches!(command.operations.get(4),
            Some(TemplateOperation::AddContent(how_are_you))));
        assert!(matches!(command.operations.get(8),
            Some(TemplateOperation::AddContent(comma))));
        assert!(matches!(command.operations.get(9),
            Some(TemplateOperation::AddValue(your_name_3))));
    }

    #[test]
    fn test_converts_template_with_tailing_chars_to_operations_correctly() {
        let template = "Hello, {{firstName}}! \
        \n My name is {{yourName1}}. \
        \n How are you doing? \
        I'm {{yourMood}}. {{yourName2}}, {{yourName3}}. \
        \n Some other content that is not a tag.";
        let command =
            AppendHtmlFromTemplateCommandBuilder::build(0, template.to_owned());
        assert_eq!(11, command.operations.len());
        let hello = "Hello, ".to_owned();
        let your_name_3 = "yourName3".to_owned();
        let end = ". \n Some other content that is not a tag.".to_owned();
        assert!(matches!(command.operations.get(0),
            Some(TemplateOperation::AddContent(hello))));
        assert!(matches!(command.operations.get(9),
            Some(TemplateOperation::AddValue(your_name_3))));
        assert!(matches!(command.operations.get(10),
            Some(TemplateOperation::AddContent(end))));
    }

    #[test]
    fn test_converts_template_with_one_char_at_the_end() {
        let template = "Hello, {{firstName}}! \
        \n My name is {{yourName1}}. \
        \n How are you doing? \
        I'm {{yourMood}}. {{yourName2}}, {{yourName3}}*";
        let command =
            AppendHtmlFromTemplateCommandBuilder::build(0, template.to_owned());
        assert_eq!(11, command.operations.len());
        let hello = "Hello, ".to_owned();
        let your_name_3 = "yourName3".to_owned();
        let end = "*".to_owned();
        assert!(matches!(command.operations.get(0),
            Some(TemplateOperation::AddContent(hello))));
        assert!(matches!(command.operations.get(9),
            Some(TemplateOperation::AddValue(your_name_3))));
        assert!(matches!(command.operations.get(10),
            Some(TemplateOperation::AddContent(end))));
    }

    #[test]
    fn test_handles_only_content() {
        let template = "Hello, \n this is some great content.".to_owned();
        let command =
            AppendHtmlFromTemplateCommandBuilder::build(0, template);
        assert_eq!(1, command.operations.len());
        assert!(matches!(command.operations.get(0),
            Some(TemplateOperation::AddContent(template))));
    }

}


