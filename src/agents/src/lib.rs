use std::future::Future;
use std::sync::{Arc, Mutex, PoisonError};

use actix::{Actor, Context, Handler, ResponseActFuture};
use futures::future::{Abortable, Aborted, AbortHandle, AbortRegistration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_bts::context::{BTNodeExecutionContext, BTNodeExecutionContextHolder};
use buttercup_bts::tick::{TickError, TickStatus};
use buttercup_bts::tree::BehaviorTree;

pub mod service;

pub struct Agent {

    id: i32,
    abort_handle: Mutex<Option<AbortHandle>>,
    context: Arc<BTNodeExecutionContextHolder>,
    tree: Arc<BehaviorTree>

}

impl Agent {

    pub fn new(id: i32,
               context: Arc<BTNodeExecutionContextHolder>,
               tree: Arc<BehaviorTree>) -> Agent {
        Agent {
            id,
            abort_handle: Mutex::default(),
            context,
            tree
        }
    }

    pub async fn start(&mut self) -> Result<TickStatus, AgentError> {
        let abort_registration = self.create_abort_registration()?;
        Result::Ok(Abortable::new(self.tree.tick(self.context.get_context()),
                                  abort_registration).await??)
    }

    pub async fn stop(&mut self) -> Result<(), AgentError> {
        match self.abort_handle.get_mut()?.take() {
            None => {}
            Some(handle) => handle.abort()
        }
        Result::Ok(())
    }

    fn create_abort_registration(&mut self) -> Result<AbortRegistration, AgentError> {
        let abort_handle_maybe = self.abort_handle.get_mut()?;
        if abort_handle_maybe.is_some() {
            return Result::Err(AgentError::AlreadyRunning);
        }
        let (abort_handle, abort_registration) =
            AbortHandle::new_pair();
        abort_handle_maybe.replace(abort_handle);
        Result::Ok(abort_registration)
    }

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum AgentError {

    AbortedError(String),
    AlreadyRunning,
    ExecutionError(TickError),
    LockPoisoned(String)

}

impl From<TickError> for AgentError {
    fn from(val: TickError) -> Self {
        AgentError::ExecutionError(val)
    }
}

impl From<Aborted> for AgentError {
    fn from(val: Aborted) -> Self {
        AgentError::AbortedError(val.to_string())
    }
}

impl From<PoisonError<&mut Option<AbortHandle>>> for AgentError {
    fn from(val: PoisonError<&mut Option<AbortHandle>>) -> Self {
        AgentError::LockPoisoned(val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use buttercup_blackboards::LocalBlackboard;
    use buttercup_bts::context::{BTNodeContextService, BTNodeExecutionContext, test_utils};
    use buttercup_bts::node::action::logging::PrintLogActionNode;
    use buttercup_bts::node::root::one_off::OneOffRootBTNode;

    use super::*;

    #[actix_rt::test]
    async fn test_returns_status() {
        let path = {
            let context: Arc<BTNodeExecutionContextHolder> =
                Arc::new(BTNodeContextService::default().build_new().unwrap());
            assert_eq!(Agent::new(1,
                                  context.clone(),
                                  Arc::new(
                                      BehaviorTree::new(1,
                                                        OneOffRootBTNode::new(
                                                            1,
                                                            PrintLogActionNode::new(
                                                                1,
                                                                "hello".to_owned())
                                                                .into()
                                                        )
                                                            .into()
                                      )
                                  )
            ).start().await,
                       Result::Ok(TickStatus::Success));
            test_utils::get_path(context.get_context())
        };

        test_utils::destroy(path);
    }

}
