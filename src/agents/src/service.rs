use std::sync::Arc;

use buttercup_bts::context::{BTNodeContextService, BTNodeContextServiceError};
use buttercup_bts::tree::BehaviorTreeService;

use crate::Agent;
use buttercup_endpoints::endpoints::EndpointService;

pub struct AgentService {

    context_service: Arc<BTNodeContextService>,
    endpoint_service: Arc<EndpointService>,
    tree_service: Arc<BehaviorTreeService>

}

impl AgentService {

    pub fn build_new_agent(&self,
                           tree_id: &i32) -> Result<Agent, AgentServiceError> {
        if let Some(tree) = self.tree_service.get_by_id(tree_id) {
            let context = self.context_service.build_new()?;

        }

        Result::Err(AgentServiceError::TreeOfIdNotFound(*tree_id))
    }

}


pub enum AgentServiceError {

    BTNodeContextServiceError(BTNodeContextServiceError),
    TreeOfIdNotFound(i32)

}

impl From<BTNodeContextServiceError> for AgentServiceError {
    fn from(err: BTNodeContextServiceError) -> Self {
        AgentServiceError::BTNodeContextServiceError(err)
    }
}