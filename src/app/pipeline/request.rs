use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ContentPipelineRequestHeader {

    id: String,
    tenant_id: String,
    pipeline_id: i32,
    created_at_utc: NaiveDateTime,

}

#[derive(Debug)]
pub struct ContentPipelineRequest<'a> {

    header: ContentPipelineRequestHeader,
    payload: &'a Value

}