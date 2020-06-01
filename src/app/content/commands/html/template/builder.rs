use crate::app::content::commands::html::template::{AppendHtmlFromTemplateCommand, TemplateOperation};
use regex::{Error, Regex};

lazy_static! {
    static ref TAG_REGEX: Regex = Regex::new(r"\{\{.*?}}").unwrap();
}

pub enum AppendHtmlFromTemplateCommandBuildError {

    RegexError

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
            println!("Found tag: {}, {}", start, end);
            current_content_start_idx = end + 1;
        }
        let template_len = template.len();
        match template_len.checked_sub(current_content_start_idx) {
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
    fn test_parsing_tags() {
        let template = "Hello, {{firstName}}! \
        \n My name is {{yourName1}}. \
        \n How are you doing? \
        I'm {{yourMood}}. {{yourName2}}, {{yourName3}}";
        AppendHtmlFromTemplateCommandBuilder::build(0, template.to_owned());
    }

}


