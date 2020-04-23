use crate::app::content::ContentType;
use serde_json::Value;
use url::Url;
use chrono::NaiveDateTime;
pub mod pipeline;

pub struct ContentPipelineDefinition {

    argument_set_definition_id: i32,
    selection_tree_definition_id: i32

}

#[derive(Debug, Clone)]
pub struct ContentPipelineRequestHeader {

    id: String,
    tenant_id: String,
    pipeline_id: i32,
    created_at_utc: NaiveDateTime,

}

pub struct ContentPipelineRequest<'a> {

    header: ContentPipelineRequestHeader,
    payload: &'a Value

}

pub struct ContentPipelineResponse {

    id: String,
    request: ContentPipelineRequestHeader,
    created_at_utc: NaiveDateTime,
    content_type: ContentType,
    url: Url

}

pub enum ContentPipelineError {

}

pub struct ContentPipelineService;

impl ContentPipelineService {

    pub fn handle(request: &ContentPipelineRequest)
        -> Result<ContentPipelineResponse, ContentPipelineError> {

    }

}

pub struct ContentPipeline;