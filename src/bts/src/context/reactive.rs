use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use futures::future::AbortHandle;
use serde::{Deserialize, Serialize};

use crate::context::BTNodeExecutionContext;
use crate::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::node::BTNode;

pub struct ReactiveService {

    abort_handles: DashMap<i32, AbortEntry>,
    nodes_by_value_names: DashMap<String, Vec<ReactiveNodeEntry>>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum ReactiveServiceError {

    AbortEntryNotFound(i32),
    AbortEntryLockError(i32, String)

}

impl ReactiveService {

    pub fn new() -> ReactiveService {
        ReactiveService { abort_handles: DashMap::new(), nodes_by_value_names: DashMap::new() }
    }

    pub fn cleanup_node(&self,
                        bt_node: &ReactiveConditionDecoratorNode) {
        let bt_node_id = bt_node.get_id();

        self.abort_handles.remove(bt_node.get_id());

        for value_name in bt_node.get_value_names() {
            self.nodes_by_value_names
                .alter(
                value_name,
                |k,v|
                    v.into_iter().filter(|entry| entry.id != *bt_node_id).collect()
            );
            self.nodes_by_value_names
                .remove_if(value_name, |k, v| v.is_empty());
        }
    }

    pub fn initialize_node(&self,
                           bt_node: Arc<BTNode>) {
        let bt_node_id = bt_node.get_id();
        let value_names = bt_node.get_value_names().clone();

        self.abort_handles.insert(*bt_node_id, AbortEntry::new(*bt_node_id));
        for value_name in value_names {
            let entry = ReactiveNodeEntry {
                id: *bt_node_id,
                node: bt_node.clone()
            };
            self.nodes_by_value_names
                .entry(value_name)
                .or_insert(vec![])
                .value_mut()
                .push(entry);
        }
    }

    pub fn register(&self,
                    bt_node_id: &i32,
                    handle: AbortHandle) -> Result<(), ReactiveServiceError> {
        match self.abort_handles.get(bt_node_id) {
            None => Result::Err(ReactiveServiceError::AbortEntryNotFound(*bt_node_id)),
            Some(entry) => match entry.push(handle) {
                Ok(_) => Result::Ok(()),
                Err(err) => Result::Err(err)
            }
        }
    }

    pub fn abort(&self,
                 bt_node_id: &i32) -> Result<(), ReactiveServiceError> {
        match self.abort_handles.get(bt_node_id) {
            None => Result::Err(ReactiveServiceError::AbortEntryNotFound(*bt_node_id)),
            Some(entry) => match entry.abort() {
                Ok(_) => Result::Ok(()),
                Err(err) => Result::Err(err)
            }
        }
    }

    pub fn handle_value_changes(&self,
                                context: &BTNodeExecutionContext,
                                changed_value_names: HashSet<String>) {
        let mut already_called = HashSet::new();

        for value_name in changed_value_names {
            if let Some(nodes) =
            self.nodes_by_value_names.get(&value_name) {
                for node in nodes.value() {
                    let node_id = node.get_id();

                    if !already_called.contains(node_id) {
                        node.get_node().handle_value_change(context);
                        already_called.insert(node_id.clone());
                    }
                }
            }
        }
    }

}

impl Default for ReactiveService {
    fn default() -> Self {
        ReactiveService::new()
    }
}

struct AbortEntry {

    bt_node_id: i32,
    handles: Mutex<Vec<AbortHandle>>

}

impl AbortEntry {

    fn new(bt_node_id: i32) -> AbortEntry {
        AbortEntry {
            bt_node_id,
            handles: Mutex::new(Vec::new())
        }
    }

    fn push(&self, handle: AbortHandle) -> Result<(), ReactiveServiceError> {
        match self.handles.lock() {
            Ok(mut handlers) => {
                handlers.push(handle);
                Result::Ok(())
            },
            Err(err) =>
                Result::Err(
                    ReactiveServiceError::AbortEntryLockError(self.bt_node_id, err.to_string()))
        }
    }

    fn abort(&self) -> Result<(), ReactiveServiceError> {
        match self.handles.lock() {
            Ok(mut handlers) => {

                for handle in handlers.iter() {
                    handle.abort();
                }
                handlers.clear();

                Result::Ok(())
            },
            Err(err) =>
                Result::Err(
                    ReactiveServiceError::AbortEntryLockError(self.bt_node_id, err.to_string()))
        }
    }

}

struct ReactiveNodeEntry {

    id: i32,
    node: Arc<BTNode>

}

impl ReactiveNodeEntry {

    fn get_id(&self) -> &i32 {
        &self.id
    }

    fn get_node(&self) -> &Arc<BTNode> {
        &self.node
    }

}