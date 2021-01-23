use std::collections::HashSet;
use std::sync::Arc;

use uuid::Uuid;

use crate::app::behavior::context::reactive::ReactiveService;
use crate::app::behavior::node::BTNode;
use crate::app::blackboards::service::{BlackboardError, BlackboardService};
use crate::app::values::{ValueHolder, ValuesPayload};

pub mod reactive;

pub struct BTNodeExecutionContext {

    blackboard_id: Uuid,
    blackboard_service: Arc<BlackboardService>,
    reactive_service: Arc<ReactiveService>

}

impl BTNodeExecutionContext {

    pub fn new(blackboard_id: Uuid,
               blackboard_service: Arc<BlackboardService>,
               reactive_service: Arc<ReactiveService>) -> BTNodeExecutionContext {
        BTNodeExecutionContext {
            blackboard_id,
            blackboard_service,
            reactive_service
        }
    }

    pub fn destroy(&self) -> Result<(), BlackboardError> {
        self.blackboard_service.destroy(&self.blackboard_id)
    }

    pub fn get_values(&self,
                      value_names: &HashSet<String>) -> Result<ValuesPayload, BlackboardError> {
        if value_names.is_empty() {
            return Result::Ok(ValuesPayload::empty());
        }
        self.blackboard_service.get_values(&self.blackboard_id, value_names)
    }

    pub fn get_value(&self,
                     value_name: &String) -> Result<Option<ValueHolder>, BlackboardError> {
        self.blackboard_service.get_value(&self.blackboard_id, value_name)
    }

    pub fn put_values(&self,
                      payload: &ValuesPayload) -> Result<(), BlackboardError> {
        self.blackboard_service.put_values(&self.blackboard_id, payload)
    }

    pub fn get_reactive_service(&self) -> &Arc<ReactiveService> {
        &self.reactive_service
    }

}

impl Default for BTNodeExecutionContext {
    fn default() -> Self {
        BTNodeExecutionContext::new(
            Uuid::new_v4(),
            Arc::new(Default::default()),
            Arc::new(Default::default()))
    }
}
