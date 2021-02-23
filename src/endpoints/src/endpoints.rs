use std::collections::HashSet;
use std::sync::Arc;

use actix::Arbiter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{BlackboardError, BlackboardService};
use buttercup_values::ValuesPayload;

pub struct EndpointService {

    blackboard_service: BlackboardService,
    value_change_handling_service: ValueChangeHandlingService

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

    pub fn accept_value_changes(&self,
                                blackboard_id: &Uuid,
                                payload: ValuesPayload) -> Result<(), EndpointError> {
        self.blackboard_service.put_values(blackboard_id, &payload)?;

        self.value_change_handling_service.handle_value_changes(payload.into_keys());

        Result::Ok(())
    }

}

pub struct ValueChangeHandlingService {

    arbiter: Arbiter,
    listener: Arc<dyn Fn(HashSet<String>) + Send + Sync>

}

impl ValueChangeHandlingService {

    fn handle_value_changes(&self,
                            keys: HashSet<String>) {
        let listener = self.listener.clone();

        self.arbiter.exec_fn(move || {
            listener(keys)
        });
    }

}