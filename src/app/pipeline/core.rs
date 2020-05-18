use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;

use crate::app::arguments::{ArgumentDefinition, ArgumentsExtractor};
use crate::app::content::definitions::ContentType;
use crate::app::selection::tree::SelectionTree;
use crate::app::transformations::Transformer;
use crate::app::transformations::transformer::TransformationRequest;
use crate::app::content::ContentCommandExecutor;

#[derive(Debug, Clone)]
pub struct ContentPipelineRequestHeader {

    id: Uuid,
    tenant_id: String,
    pipeline_id: i32,
    created_at_utc: NaiveDateTime,

}

impl ContentPipelineRequestHeader {

    pub fn new(tenant_id: String,
               pipeline_id: i32) -> ContentPipelineRequestHeader {
        ContentPipelineRequestHeader {
            id: Uuid::new_v4(),
            tenant_id,
            pipeline_id,
            created_at_utc: Utc::now().naive_utc()
        }
    }

    pub fn get_tenant_id(&self) -> &String {
        &self.tenant_id
    }

    pub fn get_pipeline_id(&self) -> &i32 {
        &self.pipeline_id
    }

}

#[derive(Debug)]
pub struct ContentPipelineRequest<'a> {

    header: ContentPipelineRequestHeader,
    payload: &'a Value

}

impl<'a> ContentPipelineRequest<'a> {

    pub fn new(tenant_id: String,
               pipeline_id: i32,
               payload: &Value) -> ContentPipelineRequest {
        ContentPipelineRequest {
            header: ContentPipelineRequestHeader::new(tenant_id, pipeline_id),
            payload
        }
    }

    pub fn get_header(&self) -> &ContentPipelineRequestHeader {
        &self.header
    }

    pub fn get_payload(&self) -> &Value {
        &self.payload
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentPipelineResponse {

    id: String,
    created_at_utc: NaiveDateTime,
    content_type: ContentType,
    external_url: String

}

pub struct ContentPipeline {

    extractor: ArgumentsExtractor,
    transformer: Transformer,
    selection_tree: SelectionTree,
    command_executor: ContentCommandExecutor

}

impl ContentPipeline {

    pub fn get_extractor(&self) -> &ArgumentsExtractor {
        &self.extractor
    }

    pub fn get_transformer(&self) -> &Transformer {
        &self.transformer
    }

    pub fn get_selection_tree(&self) -> &SelectionTree {
        &self.selection_tree
    }

}