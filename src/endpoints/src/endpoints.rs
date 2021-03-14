use std::collections::HashSet;
use std::sync::Arc;

use actix::Arbiter;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError, LocalBlackboardService};
use buttercup_values::ValuesPayload;

type Listener = Arc<dyn Fn(&HashSet<String>) + Send + Sync>;

#[derive(Default)]
pub struct EndpointService {

    arbiter: Arbiter,
    blackboard_service: Arc<LocalBlackboardService>,
    listeners: DashMap<Uuid, Listener>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum EndpointError {

    BlackboardError(LocalBlackboardError),
    LockPoisonedError

}

impl From<LocalBlackboardError> for EndpointError {
    fn from(err: LocalBlackboardError) -> Self {
        EndpointError::BlackboardError(err)
    }
}

impl EndpointService {

    pub fn new(arbiter: Arbiter,
               blackboard_service: Arc<LocalBlackboardService>) -> EndpointService {
        EndpointService {
            arbiter,
            blackboard_service,
            listeners: DashMap::new()
        }
    }

    pub fn accept_value_changes(&self,
                                blackboard_id: &Uuid,
                                payload: ValuesPayload) -> Result<(), EndpointError> {
        self.blackboard_service
            .get(blackboard_id)?
            .put_values(&payload)?;

        let keys = payload.into_keys();


        let listeners: Vec<Listener> = {
            self.listeners
                .iter()
                .map(|entry| entry.value().clone())
                .collect()
        };

        self.arbiter.exec_fn(move || {
            for listener in listeners {
                listener(&keys);
            }
        });

        Result::Ok(())
    }

    pub fn add_listener(&self,
                        listener: Arc<dyn Fn(&HashSet<String>) + Send + Sync>) {
        self.listeners.insert(Uuid::new_v4(), listener);
    }

}

