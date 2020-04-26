use dashmap::DashMap;
use std::sync::Arc;
use crate::app::transformations::Transformer;
use std::collections::HashMap;
use crate::app::arguments::ArgumentDefinition;
use crate::app::transformations::transformer::TransformationRequest;
use crate::app::selection::tree::SelectionTree;

pub struct ContentPipeline {

    transformer: Vec<TransformationRequest>,
    arguments: HashMap<String, ArgumentDefinition>,
    selection_tree: SelectionTree

}

pub struct ContentPipelineProvider {

    cache: ContentPipelineCache

}

pub struct ContentPipelineCache {

    pipelines_by_tenant_id_and_id: Arc<DashMap<String, TenantScopedContentPipelineCache>>

}

pub struct ContentPipelineRepository {



}

pub struct TenantScopedContentPipelineCache {

    pipelines_by_id: Arc<DashMap<i32, ContentPipeline>>

}