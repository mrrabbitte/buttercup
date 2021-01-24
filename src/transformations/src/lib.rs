use buttercup_values::ValuesPayload;
use serde::{Deserialize, Serialize};

use crate::transformer::{TransformationError, TransformationRequest, TransformationService};

pub mod transformer;
pub mod mono;
pub mod di;

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

