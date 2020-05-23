use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use crate::app::arguments::ArgumentDefinition;
use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValueExtractorError, ArgumentValuesExtractionService};
use crate::app::content::commands::{ContentCommandExecutionError, ContentCommandExecutorContexts};
use crate::app::content::responses::ContentCommandResponse;
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
    ContentCommandExecutionError(ContentCommandExecutionError)

}

#[derive(Clone)]
pub struct ContentPipelineEvaluationService {

    cache: ContentPipelineCache,
    selection_context: SimpleSelectionNodesContext,
    executor_contexts: ContentCommandExecutorContexts

}

impl ContentPipelineEvaluationService {

    pub fn new(cache: ContentPipelineCache,
               selection_context: SimpleSelectionNodesContext,
               executor_contexts: ContentCommandExecutorContexts)
        -> ContentPipelineEvaluationService {
        ContentPipelineEvaluationService {
            cache,
            selection_context,
            executor_contexts
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
        match pipeline.get_command_executor().execute(&self.executor_contexts,
                                                &values,
                                                decision.get_content_commands()) {
            Ok(result) =>
                Result::Ok(
                    ContentPipelineResponse::new(header.clone(),
                                                 decision.get_id().clone(),
                                                 result.get_path().clone())),
            Err(err) =>
                Result::Err(
                    ContentPipelineEvaluationError::ContentCommandExecutionError(err)),
        }

    }

}



