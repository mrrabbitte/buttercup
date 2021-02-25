use std::collections::HashSet;
use std::sync::Arc;

use actix::Arbiter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{BlackboardError, BlackboardService};
use buttercup_values::ValuesPayload;

pub struct EndpointService {

    arbiter: Arbiter,
    blackboard_service: Arc<BlackboardService>,
    listener: Arc<dyn Fn(HashSet<String>) + Send + Sync>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum EndpointError {

    BlackboardError(BlackboardError)

}

impl From<BlackboardError> for EndpointError {
    fn from(err: BlackboardError) -> Self {
        EndpointError::BlackboardError(err)
    }
}

impl EndpointService {

    pub fn new(arbiter: Arbiter,
               blackboard_service: Arc<BlackboardService>,
               listener: Arc<dyn Fn(HashSet<String>) + Send + Sync>) -> EndpointService {
        EndpointService {
            arbiter,
            blackboard_service,
            listener
        }
    }

    pub fn accept_value_changes(&self,
                                blackboard_id: &Uuid,
                                payload: ValuesPayload) -> Result<(), EndpointError> {
        self.blackboard_service.put_values(blackboard_id, &payload)?;

        let keys = payload.into_keys();

        let listener = self.listener.clone();

        self.arbiter.exec_fn(move || {
            listener(keys)
        });

        Result::Ok(())
    }

}

