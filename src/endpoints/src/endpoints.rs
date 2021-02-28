use std::collections::HashSet;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use actix::Arbiter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError, LocalBlackboardService};
use buttercup_values::ValuesPayload;

type Listeners = Vec<Arc<dyn Fn(&HashSet<String>) + Send + Sync>>;

pub struct EndpointService {

    arbiter: Arbiter,
    blackboard_service: Arc<LocalBlackboardService>,
    listeners: RwLock<Listeners>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum EndpointError {

    BlackboardError(LocalBlackboardError),
    LockPoisonedError

}

impl From<PoisonError<RwLockReadGuard<'_, Listeners>>> for EndpointError {
    fn from(_: PoisonError<RwLockReadGuard<'_, Listeners>>) -> Self {
        EndpointError::LockPoisonedError
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, Listeners>>> for EndpointError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, Listeners>>) -> Self {
        EndpointError::LockPoisonedError
    }
}

impl From<LocalBlackboardError> for EndpointError {
    fn from(err: LocalBlackboardError) -> Self {
        EndpointError::BlackboardError(err)
    }
}

impl EndpointService {

    pub fn new(arbiter: Arbiter,
               blackboard_service: Arc<LocalBlackboardService>,
               listeners: Listeners) -> EndpointService {
        EndpointService {
            arbiter,
            blackboard_service,
            listeners: RwLock::new(listeners)
        }
    }

    pub fn accept_value_changes(&self,
                                blackboard_id: &Uuid,
                                payload: ValuesPayload) -> Result<(), EndpointError> {
        self.blackboard_service
            .get(blackboard_id)?
            .put_values(&payload)?;

        let keys = payload.into_keys();


        let listeners = {
            self.listeners.read()?.clone()
        };

        self.arbiter.exec_fn(move || {
            for listener in listeners {
                listener(&keys);
            }
        });

        Result::Ok(())
    }

    pub fn add_listener(&mut self,
                        listener: Arc<dyn Fn(&HashSet<String>) + Send + Sync>)
        -> Result<(), EndpointError> {
        self.listeners.write()?.push(listener);

        Result::Ok(())
    }

}

