use std::future::Future;
use std::sync::{Arc, Mutex, PoisonError};

use actix::{Actor, Context, Handler, ResponseActFuture};
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::future::{Abortable, Aborted, AbortHandle, AbortRegistration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_bts::context::{BTNodeExecutionContext, BTNodeExecutionContextHolder};
use buttercup_bts::tick::{TickError, TickStatus};
use buttercup_bts::tree::BehaviorTree;

pub mod service;

pub struct Agent {

    id: Uuid,
    context: Arc<BTNodeExecutionContextHolder>,
    tree: Arc<BehaviorTree>

}

impl Agent {

    pub fn new(id: Uuid,
               context: Arc<BTNodeExecutionContextHolder>,
               tree: Arc<BehaviorTree>) -> Agent {
        Agent {
            id,
            context,
            tree
        }
    }

    pub async fn start(&self,
                       abort_registration: AbortRegistration) -> AgentExecutionResult {
        let exec_id = Uuid::new_v4();
        let started_at = Utc::now();

        let result = self.do_start(abort_registration).await;

        AgentExecutionResult::new(
            exec_id,
            self.id.clone(),
            Utc::now().naive_utc(),
            result,
            started_at.naive_utc())
    }

    async fn do_start(&self,
                      abort_registration: AbortRegistration) -> Result<TickStatus, AgentError> {
        Result::Ok(
            Abortable::new(
                self.tree.tick(
                    Uuid::new_v4(),
                    self.context.get_context()), abort_registration)
                .await??)
    }

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum AgentError {

    AbortedError(String),
    AlreadyRunning,
    ExecutionError(TickError)

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

pub struct AgentExecutionResult {

    id: Uuid,
    agent_id: Uuid,
    ended_at_utc: NaiveDateTime,
    result: Result<TickStatus, AgentError>,
    started_at_utc: NaiveDateTime

}

impl AgentExecutionResult {

    pub fn new(id: Uuid,
               agent_id: Uuid,
               ended_at_utc: NaiveDateTime,
               result: Result<TickStatus, AgentError>,
               started_at_utc: NaiveDateTime) -> AgentExecutionResult {
        AgentExecutionResult {
            id,
            agent_id,
            ended_at_utc,
            result,
            started_at_utc
        }
    }

}



#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_rt::System;
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

            let mut agent = Agent::new(Uuid::new_v4(),
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
            );
            let (abort_handle, abort_registration) =
                AbortHandle::new_pair();
            let result = agent.start(abort_registration).await;

            assert_eq!(result.result.unwrap(), TickStatus::Success);

            test_utils::get_path(context.get_context())
        };

        test_utils::destroy(path);
    }

}
