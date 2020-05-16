use crate::app::decision::SelectionDecisionService;
use crate::app::pipeline::ContentPipelineService;
use crate::app::pipeline::evaluation::cache::ContentPipelineCache;
use crate::app::pipeline::evaluation::ContentPipelineEvaluationService;
use crate::app::reinforcement::ReinforcementService;
use crate::app::selection::nodes::context::SimpleSelectionNodesContext;

pub fn content_pipeline_service() -> ContentPipelineService {
    ContentPipelineService::new(
        ContentPipelineEvaluationService::new(
            ContentPipelineCache::new(),
            SimpleSelectionNodesContext::new(
                ReinforcementService::new(
                    SelectionDecisionService::new()))
        )
    )
}