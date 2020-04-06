use crate::app::transformations::di::astro::IsDay;
use crate::app::transformations::di::DiInputTransformation;
use crate::app::transformations::mono::MonoInputTransformation;
use crate::app::transformations::transformer::{DoubleInputTransformationDefinition, SingleInputTransformationDefinition, TransformationDefinition, TransformationError, TransformationRequest, TransformationService, TransformationType};
use crate::app::values::ValuesPayload;

pub mod transformer;
pub mod mono;
pub mod di;


pub struct Transformer {

    requests: Vec<TransformationRequest>

}

impl Transformer {

    pub fn transform(&self,
                     payload: &ValuesPayload) -> Result<ValuesPayload, TransformationError> {
        TransformationService::transform(payload, &self.requests)
    }

}


