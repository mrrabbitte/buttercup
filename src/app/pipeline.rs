use chrono::NaiveDateTime;
use serde_json::Value;
use url::Url;

use crate::app::content::ContentType;
use crate::app::pipeline::request::ContentPipelineRequest;
use crate::app::pipeline::response::ContentPipelineResponse;
use crate::app::selection::nodes::SelectionNodeError;
use crate::app::selection::tree::SelectionTreeError;

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
        unimplemented!()
    }

}

pub struct ContentPipeline;