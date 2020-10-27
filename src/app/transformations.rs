
use crate::app::values::ValuesPayload;

pub mod transformer;
pub mod mono;
pub mod di;

use serde::{Serialize, Deserialize};
use crate::app::transformations::transformer::{TransformationRequest, TransformationService, TransformationError};

#[derive(Serialize, Deserialize)]
pub struct Transformer {

    requests: Vec<TransformationRequest>

}

impl Transformer {

    pub fn new(requests: Vec<TransformationRequest>) -> Transformer {
        Transformer {
            requests
        }
    }

    pub fn transform(&self,
                     payload: &ValuesPayload) -> Result<ValuesPayload, TransformationError> {
        TransformationService::transform(payload, &self.requests)
    }

}


