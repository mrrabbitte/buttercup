use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use actix::Arbiter;
use dashmap::DashMap;
use futures::future::AbortHandle;
use futures::io::Error;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use uuid::Uuid;

use buttercup_bts::context::{BTNodeContextService, BTNodeContextServiceError};
use buttercup_bts::tree::BehaviorTreeService;
use buttercup_endpoints::endpoints::EndpointService;

use crate::Agent;
use crate::service::AgentServiceError::AgentAlreadyStarted;

pub struct AgentService {

    context_service: Arc<BTNodeContextService>,
    started_agents: DashMap<Uuid, (Arc<Agent>, AbortHandle)>,
    stopped_agents: DashMap<Uuid, Arc<Agent>>,
    tree_service: Arc<BehaviorTreeService>,
    runtime: Runtime

}

impl AgentService {

    pub fn new(context_service: Arc<BTNodeContextService>,
               tree_service: Arc<BehaviorTreeService>) -> Result<AgentService, AgentServiceError> {
        Result::Ok(
            AgentService {
                stopped_agents: DashMap::new(),
                started_agents: DashMap::new(),
                context_service,
                tree_service,
                runtime: Runtime::new()?
            }
        )
    }

    pub fn build_new_agent(&self,
                           tree_id: &i32) -> Result<Uuid, AgentServiceError> {
        if let Some(tree) = self.tree_service.get_by_id(tree_id) {
            let context =
                Arc::new(self.context_service.build_new()?);

            let agent_id = Uuid::new_v4();
            self.stopped_agents.insert(agent_id,
                                       Arc::new(
                                           Agent::new(agent_id.clone(), context, tree)));

            return Result::Ok(agent_id);
        }

        Result::Err(AgentServiceError::TreeOfGivenIdNotFound(*tree_id))
    }

    pub fn start_agent_by_id(&self,
                             agent_id: &Uuid) -> Result<(), AgentServiceError> {
        match self.stopped_agents.remove(agent_id) {
            None => {
                if self.started_agents.contains_key(agent_id) {
                    return Result::Err(AgentServiceError::AgentAlreadyStarted);
                }
                return Result::Err(AgentServiceError::AgentOfGivenIdNotFound);
            }
            Some(agent_entry) => {
                let agent = agent_entry.1;
                let (abort_handle, abort_registration) =
                    AbortHandle::new_pair();

                let agent_ref = agent.clone();

                self.runtime.spawn(async move {
                    agent.start(abort_registration).await
                });

                self.started_agents.insert(agent_id.clone(), (agent_ref, abort_handle));
            }
        }

        Result::Ok(())
    }

    pub fn stop_agent_by_id(&self,
                            agent_id: &Uuid) -> Result<(), AgentServiceError> {
        match self.started_agents.remove(agent_id) {
            None => {
                return Result::Err(AgentServiceError::AgentOfGivenIdNotFound);
            }
            Some(agent_entry) => {
                let (agent, abort_handle) = agent_entry.1;
                abort_handle.abort();

                self.stopped_agents.insert(agent_entry.0, agent);
            }
        }

        Result::Ok(())
    }
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum AgentServiceError {

    AgentAlreadyStarted,
    AgentOfGivenIdNotFound,
    BTNodeContextServiceError(BTNodeContextServiceError),
    IOError(String),
    TreeOfGivenIdNotFound(i32)

}

impl From<BTNodeContextServiceError> for AgentServiceError {
    fn from(err: BTNodeContextServiceError) -> Self {
        AgentServiceError::BTNodeContextServiceError(err)
    }
}

impl From<std::io::Error> for AgentServiceError {
    fn from(err: Error) -> Self {
        AgentServiceError::IOError(err.to_string())
    }
}