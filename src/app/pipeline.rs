use crate::app::content::ContentType;
use serde_json::Value;
use url::Url;
use chrono::NaiveDateTime;
use crate::app::selection::tree::SelectionTreeError;
use crate::app::selection::nodes::SelectionNodeError;
use crate::app::pipeline::response::ContentPipelineResponse;
use crate::app::pipeline::request::ContentPipelineRequest;

pub mod evaluation;
pub mod definitions;
pub mod request;
pub mod response;



pub enum ContentPipelineError {

    SelectionTreeError(SelectionTreeError)

}

pub struct ContentPipelineEvaluationService;

impl ContentPipelineEvaluationService {

    pub fn handle(request: &ContentPipelineRequest)
        -> Result<ContentPipelineResponse, ContentPipelineError> {
        Result::Err(ContentPipelineError::SelectionTreeError(
            SelectionTreeError::SelectionNodeError(
                SelectionNodeError::SimpleSelectionError)))
    }

}

pub struct ContentPipeline;