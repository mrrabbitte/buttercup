use crate::app::pipeline::core::{ContentPipelineRequest, ContentPipelineResponse};
use crate::app::pipeline::evaluation::{ContentPipelineEvaluationError, ContentPipelineEvaluationService};

pub mod core;
pub mod definitions;
pub mod evaluation;

#[derive(Clone)]
pub struct ContentPipelineService {

    evaluation_service: ContentPipelineEvaluationService

}

impl ContentPipelineService {

    pub fn new(evaluation_service: ContentPipelineEvaluationService) -> ContentPipelineService {
        ContentPipelineService {
            evaluation_service
        }
    }

    pub fn evaluate(&self,
                    request: &ContentPipelineRequest)
        -> Result<ContentPipelineResponse, ContentPipelineEvaluationError> {
        self.evaluation_service.handle(request)
    }

}


