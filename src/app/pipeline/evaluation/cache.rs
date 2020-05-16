use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use crate::app::pipeline::core::ContentPipeline;

#[derive(Clone)]
pub struct ContentPipelineCache {

    pipelines: Arc<DashMap<ContentPipelineKey, ContentPipeline>>

}

impl ContentPipelineCache {

    pub fn new() -> ContentPipelineCache {
        ContentPipelineCache {
            pipelines: Arc::new(DashMap::new())
        }
    }

}

#[derive(PartialEq, Hash, Eq)]
pub struct ContentPipelineKey {

    tenant_id: String,
    pipeline_id: i32,

}

pub enum ContentPipelineCacheError {

    NotFound

}

impl ContentPipelineKey {

    pub fn new(tenant_id: String,
               pipeline_id: i32) -> ContentPipelineKey {
        ContentPipelineKey {
            tenant_id,
            pipeline_id
        }
    }

}

impl ContentPipelineCache {

    pub fn get(&self,
               tenant_id: &String,
               pipeline_id: &i32)
        -> Result<Ref<ContentPipelineKey, ContentPipeline>, ContentPipelineCacheError> {
        let key =
            ContentPipelineKey::new(tenant_id.clone(), *pipeline_id);
        match self.pipelines.get(&key) {
            None => Result::Err(ContentPipelineCacheError::NotFound),
            Some(pipeline) => Result::Ok(pipeline),
        }
    }

    pub fn put(&self,
               tenant_id: &String,
               pipeline_id: &i32,
               pipeline: ContentPipeline) {
        self.pipelines.insert(
            ContentPipelineKey::new(tenant_id.clone(), *pipeline_id),
            pipeline);
    }

}
