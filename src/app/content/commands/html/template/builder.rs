use crate::app::content::commands::html::template::{AppendHtmlFromTemplateCommand, TemplateOperation};
use regex::bytes::Regex;


pub enum AppendHtmlFromTemplateCommandBuildError {

    RegexError

}

pub(crate) struct AppendHtmlFromTemplateCommandBuild;

impl AppendHtmlFromTemplateCommandBuild {

    pub fn build(id: i32,
                 template: String)
        -> Result<AppendHtmlFromTemplateCommand, AppendHtmlFromTemplateCommandBuildError> {
        let template_bytes = template.as_bytes();
        let re = Regex::new(r"(\{\{.*}})").unwrap();
        let cap = re.captures(template.as_bytes()).unwrap();
        Result::Err(AppendHtmlFromTemplateCommandBuildError::RegexError)
    }

}


