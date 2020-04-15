use crate::app::content::ContentType;
use serde_json::Value;
use url::Url;
use chrono::NaiveDateTime;


pub struct SelectionTreeArguments {

    argument_definition_id: i32,
    selection_tree_definition_id: i32

}

pub struct ContentPipelineRequest<'a> {

    tenant_id: String,
    pipeline_id: i32,
    content_type: ContentType,
    payload: &'a Value

}

pub struct ContentPipelineResponse {

    pipeline_id: i32,
    created_at_utc: NaiveDateTime,
    content_type: ContentType,
    url: Url,

}

pub enum ContentPipelineError {



}

pub struct ContentPipelineService;

impl ContentPipelineService {



}