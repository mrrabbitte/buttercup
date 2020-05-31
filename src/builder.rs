use crate::app::content::commands::ContentCommandExecutorContexts;
use crate::app::content::commands::html::HtmlContentCommandsContext;
use crate::app::content::commands::video::VideoContentCommandsContext;
use crate::app::decision::SelectionDecisionService;
use crate::app::pipeline::ContentPipelineService;
use crate::app::pipeline::evaluation::cache::ContentPipelineCache;
use crate::app::pipeline::evaluation::ContentPipelineEvaluationService;
use crate::app::reinforcement::ReinforcementService;
use crate::app::selection::nodes::context::SimpleSelectionNodesContext;
use crate::test_utils::TestUtils;
use crate::app::files::FileService;
use std::collections::HashMap;

pub fn content_pipeline_service() -> ContentPipelineService {
    let cache = ContentPipelineCache::new();
    let test_pipeline = TestUtils::test_pipeline();
    cache.put(
        test_pipeline.get_tenant_id().clone(),
        test_pipeline.get_id().clone(),
        TestUtils::test_pipeline());
    ContentPipelineService::new(
        ContentPipelineEvaluationService::new(
            cache,
            SimpleSelectionNodesContext::new(
                ReinforcementService::new(
                    SelectionDecisionService::new())),
            ContentCommandExecutorContexts::new(
                HtmlContentCommandsContext::new(
                    TestUtils::test_file_service()),
                VideoContentCommandsContext::new())
        )
    )
}