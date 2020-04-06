use crate::app::content::ContentType;
use serde_json::Value;
use url::Url;


pub struct SelectionTreeArguments {

    argument_definition_id: i32,
    selection_tree_definition_id: i32

}

pub struct ContentPipelineRequest<'a> {

    tenant_id: String,
    selection_tree_id: i32,
    content_type: ContentType,
    payload: &'a Value

}

pub struct ContentPipelineResponse {

    selection_tree_id: i32,
    url: Url,

}

pub enum ContentPipelineError {



}

pub struct ContentPipelineService;

impl ContentPipelineService {



}