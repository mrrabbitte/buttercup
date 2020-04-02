use crate::app::content::ContentType;
use serde_json::Value;

pub struct ContentSelectionPipelineService;

pub struct SelectionTreeArguments {

    argument_definition_id: i32,
    selection_tree_definition_id: i32

}

pub struct ContentSelectionRequest<'a> {

    tenant_id: String,
    selection_tree_id: i32,
    content_type: ContentType,
    payload: &'a Value

}

pub enum ContentSelectionError {



}

impl ContentSelectionPipelineService {

    pub fn handle(request: ContentSelectionRequest) -> {

    }

}