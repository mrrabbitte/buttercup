use std::collections::HashSet;
use std::ffi::OsString;
use std::sync::Arc;
use log::info;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError, LocalBlackboardService};
use buttercup_values::{ValueHolder, ValuesPayload};
use buttercup_variables::{VariableName, VariableService, VariableServiceErrorReport, VariableValueAccessError};

use crate::context::reactive::ReactiveContext;
use crate::node::BTNode;
use buttercup_endpoints::endpoints::EndpointService;
use crate::events::{BTNodeExecutionEndedEvent, BTNodeExecutionStartedEvent};

pub mod reactive;

pub struct BTNodeExecutionContextHolder {

    id: Uuid,
    context: Arc<BTNodeExecutionContext>,
    value_changes_listener: Arc<dyn Fn(&HashSet<String>) + Send + Sync>

}

impl BTNodeExecutionContextHolder {

    pub fn new(id: Uuid,
               local_blackboard: Arc<LocalBlackboard>,
               reactive_service: Arc<ReactiveContext>) -> BTNodeExecutionContextHolder {
        let context =
            Arc::new(
                BTNodeExecutionContext::new(
                    local_blackboard,
                    reactive_service.clone()));

        BTNodeExecutionContextHolder {
            id,
            context: context.clone(),
            value_changes_listener: Arc::new(move |changed|
                reactive_service.handle_value_changes(
                    context.as_ref(), changed))
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_context(&self) -> &BTNodeExecutionContext {
        self.context.as_ref()
    }

    pub fn get_value_changes_listener(&self) -> Arc<dyn Fn(&HashSet<String>) + Send + Sync> {
        self.value_changes_listener.clone()
    }

}

pub struct BTNodeExecutionContext {

    local_blackboard: Arc<LocalBlackboard>,
    reactive_service: Arc<ReactiveContext>,

}

impl BTNodeExecutionContext {

    pub fn new(local_blackboard: Arc<LocalBlackboard>,
               reactive_service: Arc<ReactiveContext>) -> BTNodeExecutionContext {
        BTNodeExecutionContext {
            local_blackboard,
            reactive_service
        }
    }

    pub async fn consume_execution_started_event(&self,
                                                 event: BTNodeExecutionStartedEvent<'_>) {
        info!("{:?}", event)
    }

    pub async fn consume_execution_ended_event(&self,
                                               event: BTNodeExecutionEndedEvent<'_>) {
        info!("{:?}", event)
    }

    pub fn get_reactive_service(&self) -> &Arc<ReactiveContext> {
        &self.reactive_service
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
            Arc::new(
                LocalBlackboard::new(format!("{}.bb", Uuid::new_v4()).into()).unwrap()),
            Arc::new(Default::default()))
    }
}
#[derive(Default)]
pub struct BTNodeContextService {

    contexts: DashMap<Uuid, Arc<BTNodeExecutionContextHolder>>,
    endpoint_service: Arc<EndpointService>,
    local_blackboard_service: Arc<LocalBlackboardService>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum BTNodeContextServiceError {

    LocalBlackboardError(LocalBlackboardError)

}

impl From<LocalBlackboardError> for BTNodeContextServiceError {
    fn from(err: LocalBlackboardError) -> Self {
        BTNodeContextServiceError::LocalBlackboardError(err)
    }
}

impl BTNodeContextService {

    pub fn new(endpoint_service: Arc<EndpointService>,
               local_blackboard_service: Arc<LocalBlackboardService>) -> BTNodeContextService {
        BTNodeContextService {
            contexts: DashMap::new(),
            endpoint_service,
            local_blackboard_service
        }
    }

    pub fn build_new(&self) -> Result<BTNodeExecutionContextHolder, BTNodeContextServiceError> {
        let uuid = Uuid::new_v4();
        let blackboard_service =
            self.local_blackboard_service.create(
                &uuid, format!("{}.bb", &uuid).into())?;

        let holder = BTNodeExecutionContextHolder::new(
            uuid,
            blackboard_service,
            Arc::new(ReactiveContext::new()));

        self.endpoint_service.add_listener(holder.get_value_changes_listener());

        Result::Ok(holder)
    }

    pub fn insert(&self,
                  context: BTNodeExecutionContextHolder) {
        self.contexts.insert(context.id, Arc::new(context));
    }

    pub fn get_by_id(&self,
                     id: &Uuid) -> Option<Arc<BTNodeExecutionContextHolder>> {
        self.contexts
            .get(id)
            .map(|context_arc| context_arc.clone())
    }

}

pub mod test_utils {
    use std::ffi::OsString;

    use buttercup_blackboards::LocalBlackboard;

    use crate::context::BTNodeExecutionContext;

    pub fn cleanup(context: &BTNodeExecutionContext) {
        destroy(get_path(context));
    }

    pub fn destroy(path: OsString) {
        LocalBlackboard::destroy(path).unwrap();
    }

    pub fn get_path(context: &BTNodeExecutionContext) -> OsString {
        context.local_blackboard.get_path().unwrap()
    }

}
