use uuid::Uuid;
use std::sync::Arc;
use crate::app::blackboards::service::{BlackboardService, BlackboardError};
use crate::app::values::ValuesPayload;
use crate::app::behavior::node::{BTNodeAddress, BTNode};
use std::collections::HashSet;

pub struct BTNodeExecutionContext {

    blackboard_id: Uuid,
    blackboard_service: Arc<BlackboardService>

}

impl BTNodeExecutionContext {

    pub fn new(blackboard_id: Uuid,
               blackboard_service: Arc<BlackboardService>) -> BTNodeExecutionContext {
        BTNodeExecutionContext {
            blackboard_id,
            blackboard_service
        }
    }

    pub fn destroy(&self) -> Result<(), BlackboardError> {
        self.blackboard_service.destroy(&self.blackboard_id)
    }

    pub fn get_values(&self,
                      value_names: &HashSet<String>) -> Result<ValuesPayload, BlackboardError> {
        self.blackboard_service.get_values(&self.blackboard_id, value_names)
    }

    pub fn put_values(&self,
                      payload: &ValuesPayload) -> Result<(), BlackboardError> {
        self.blackboard_service.put_values(&self.blackboard_id, payload)
    }

}