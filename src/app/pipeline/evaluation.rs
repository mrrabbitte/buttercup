use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use crate::app::arguments::ArgumentDefinition;
use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValueExtractorError, ArgumentValuesExtractionService};
use crate::app::decision::SelectionDecision;
use crate::app::pipeline::core::{ContentPipeline, ContentPipelineRequest, ContentPipelineRequestHeader, ContentPipelineResponse};
use crate::app::pipeline::evaluation::cache::{ContentPipelineCache, ContentPipelineCacheError, ContentPipelineKey};
use crate::app::selection::nodes::context::SimpleSelectionNodesContext;
use crate::app::selection::tree::{SelectionTree, SelectionTreeError};
use crate::app::transformations::Transformer;
use crate::app::transformations::transformer::{TransformationError, TransformationRequest};
use crate::app::values::ValuesPayload;

pub mod cache;

#[derive(Debug)]
pub enum ContentPipelineEvaluationError {

    ContentPipelineCacheError(ContentPipelineCacheError),
    ArgumentValueExtractorError(ArgumentValueExtractorError),
    TransformationError(TransformationError),
    SelectionTreeError(SelectionTreeError),
    ContentCommandsError

}

#[derive(Clone)]
pub struct ContentPipelineEvaluationService {

    cache: ContentPipelineCache,
    selection_context: SimpleSelectionNodesContext

}

impl ContentPipelineEvaluationService {

    pub fn new(cache: ContentPipelineCache,
               selection_context: SimpleSelectionNodesContext) -> ContentPipelineEvaluationService {
        ContentPipelineEvaluationService {
            cache,
            selection_context
        }
    }

    pub fn handle(&self,
                  request: &ContentPipelineRequest)
                  -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        let header = request.get_header();
        match self.cache.get(header.get_tenant_id(), header.get_pipeline_id()) {
            Ok(pipeline) => self.extract(request, &pipeline),
            Err(err) =>
                Result::Err(
                    ContentPipelineEvaluationError::ContentPipelineCacheError(err)),
        }
    }

    fn extract(&self,
               request: &ContentPipelineRequest,
               pipeline: &ContentPipeline)
               -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        match pipeline.get_extractor().extract(request.get_payload()) {
            Ok(values) => self.transform(request.get_header(), pipeline, values),
            Err(err) => Result::Err(
                ContentPipelineEvaluationError::ArgumentValueExtractorError(err)
            ),
        }
    }

    fn transform(&self,
                 header: &ContentPipelineRequestHeader,
                 pipeline: &ContentPipeline,
                 values: ValuesPayload)
                 -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        match pipeline.get_transformer().transform(&values) {
            Ok(values) => self.select(header, pipeline, values),
            Err(err) => Result::Err(
                ContentPipelineEvaluationError::TransformationError(err)
            ),
        }
    }

    fn select(&self,
              header: &ContentPipelineRequestHeader,
              pipeline: &ContentPipeline,
              values: ValuesPayload)
              -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        match pipeline.get_selection_tree().evaluate(&values, &self.selection_context) {
            Ok(decision) => self.execute_commands(header, pipeline, values, decision),
            Err(err) => Result::Err(
                ContentPipelineEvaluationError::SelectionTreeError(err)
            ),
        }
    }

    fn execute_commands(&self,
                        header: &ContentPipelineRequestHeader,
                        pipeline: &ContentPipeline,
                        values: ValuesPayload,
                        decision: SelectionDecision)
        -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        println!("{:?}", decision);
        Result::Err(ContentPipelineEvaluationError::ContentCommandsError)
    }

}



