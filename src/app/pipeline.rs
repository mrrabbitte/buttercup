use crate::app::pipeline::core::{ContentPipelineRequest, ContentPipelineResponse};
use crate::app::pipeline::evaluation::{ContentPipelineEvaluationError, ContentPipelineEvaluationService};
use std::time::Instant;

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
        let t0 = Instant::now();
        let result = self.evaluation_service.handle(request);
        format!("Took: {} [ms]", t0.elapsed().as_millis());
        return result;
    }

}


