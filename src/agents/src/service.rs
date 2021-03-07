use std::sync::{Arc, Mutex};

use actix::Arbiter;
use actix_web::dev::Service;
use dashmap::DashMap;
use futures::io::Error;
use tokio::runtime::Runtime;
use uuid::Uuid;

use buttercup_bts::context::{BTNodeContextService, BTNodeContextServiceError};
use buttercup_bts::tree::BehaviorTreeService;
use buttercup_endpoints::endpoints::EndpointService;

use crate::Agent;

pub struct AgentService {

    agents: DashMap<Uuid, Mutex<Agent>>,
    context_service: Arc<BTNodeContextService>,
    endpoint_service: Arc<EndpointService>,
    tree_service: Arc<BehaviorTreeService>,
    runtime: Runtime

}

impl AgentService {

    pub fn new(context_service: Arc<BTNodeContextService>,
               endpoint_service: Arc<EndpointService>,
               tree_service: Arc<BehaviorTreeService>) -> Result<AgentService, AgentServiceError> {
        Result::Ok(
            AgentService {
                agents: DashMap::new(),
                context_service,
                endpoint_service,
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
            self.agents.insert(agent_id,
                               Mutex::new(Agent::new(agent_id.clone(), context, tree)));

            return Result::Ok(agent_id);
        }

        Result::Err(AgentServiceError::TreeOfGivenIdNotFound(*tree_id))
    }

    pub fn start_agent_by_id(&self,
                             agent_id: &Uuid) -> Result<(), AgentServiceError> {
        // self.agents
        //     .alter(agent_id,
        //            |id, agent| {
        //                let future = agent.lock().unwrap().start();
        //
        //                //self.runtime.spawn(future);
        //
        //                agent
        //            }
        //     );

        Result::Ok(())
    }

}


pub enum AgentServiceError {

    BTNodeContextServiceError(BTNodeContextServiceError),
    IOError(std::io::Error),
    TreeOfGivenIdNotFound(i32)

}

impl From<BTNodeContextServiceError> for AgentServiceError {
    fn from(err: BTNodeContextServiceError) -> Self {
        AgentServiceError::BTNodeContextServiceError(err)
    }
}

impl From<std::io::Error> for AgentServiceError {
    fn from(err: Error) -> Self {
        AgentServiceError::IOError(err)
    }
}