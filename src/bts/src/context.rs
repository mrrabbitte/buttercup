use std::collections::HashSet;
use std::sync::Arc;

use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError};
use buttercup_values::{ValueHolder, ValuesPayload};
use buttercup_variables::{VariableName, VariableService, VariableServiceErrorReport, VariableValueAccessError};

use crate::context::reactive::ReactiveContext;
use crate::node::BTNode;

pub mod reactive;

pub struct BTNodeExecutionContext {

    blackboard_id: Uuid,
    local_blackboard: Arc<LocalBlackboard>,
    reactive_service: Arc<ReactiveContext>

}

impl BTNodeExecutionContext {

    pub fn new(blackboard_id: Uuid,
               local_blackboard: Arc<LocalBlackboard>,
               reactive_service: Arc<ReactiveContext>) -> BTNodeExecutionContext {
        BTNodeExecutionContext {
            blackboard_id,
            local_blackboard,
            reactive_service
        }
    }

    pub fn get_values(&self,
                      value_names: &HashSet<String>) -> Result<ValuesPayload, LocalBlackboardError> {
        if value_names.is_empty() {
            return Result::Ok(ValuesPayload::empty());
        }

        self.local_blackboard.get_values(value_names)
    }

    pub fn get_value(&self,
                     value_name: &String) -> Result<Option<ValueHolder>, LocalBlackboardError> {
        self.local_blackboard.get_value(value_name)
    }

    pub fn put_values(&self,
                      payload: &ValuesPayload) -> Result<(), LocalBlackboardError> {
        self.local_blackboard.put_values(payload)
    }

    pub fn get_reactive_service(&self) -> &Arc<ReactiveContext> {
        &self.reactive_service
    }

    fn map_err(err: LocalBlackboardError) -> VariableValueAccessError {
        VariableValueAccessError::VariableServiceError(
            VariableServiceErrorReport::new(
                "Blackboard error".to_owned(),
                format!("{:?}", err)))
    }
}

impl VariableService for BTNodeExecutionContext {
    fn get_variable_value_by_name(&self,
                                  name: &VariableName)
                                  -> Result<Option<ValueHolder>, VariableValueAccessError> {
        self.local_blackboard
            .get_value(name.get_value())
            .map_err(BTNodeExecutionContext::map_err)
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

