use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use dashmap::{DashMap, DashSet};
use dashmap::mapref::one::Ref;
use futures::future::AbortHandle;
use serde::{Deserialize, Serialize};

use crate::context::BTNodeExecutionContext;
use crate::node::BTNode;
use crate::node::decorator::reactive::{ReactiveConditionDecoratorNode, ReactiveConditionInnerNode};
use std::ops::Deref;

#[derive(Default)]
pub struct ReactiveContext {

    abort_handles: DashMap<i32, AbortHandle>,
    nodes_by_value_names: DashMap<String, DashSet<Arc<ReactiveConditionInnerNode>>>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum ReactiveContextError {

    AbortEntryNotFound(i32)

}

impl ReactiveContext {

    pub fn new() -> ReactiveContext {
        ReactiveContext { abort_handles: DashMap::new(), nodes_by_value_names: DashMap::new() }
    }

    pub fn abort(&self,
                 bt_node_id: &i32) -> Result<(), ReactiveContextError> {
        match self.abort_handles.get(bt_node_id) {
            None => Result::Err(ReactiveContextError::AbortEntryNotFound(*bt_node_id)),
            Some(entry) => Result::Ok(entry.value().abort())
        }
    }

    pub fn deregister(&self,
                      bt_node: &ReactiveConditionInnerNode) {
        let bt_node_id = bt_node.get_id();

        self.abort_handles.remove(bt_node.get_id());

        for value_name in bt_node.get_value_names() {
            self.nodes_by_value_names
                .alter(
                    value_name,
                    |k,v|
                        v
                            .into_iter()
                            .filter(
                                |node|
                                    node.get_id() != bt_node_id)
                            .collect()
                );
            self.nodes_by_value_names
                .remove_if(
                    value_name,
                    |k, v| v.is_empty());
        }
    }

    pub fn handle_value_changes(&self,
                                context: &BTNodeExecutionContext,
                                changed_value_names: &HashSet<String>) {
        let mut already_called = HashSet::new();

        for value_name in changed_value_names {
            if let Some(nodes) =
            self.nodes_by_value_names.get(value_name) {
                for node_entry in nodes.value().iter() {
                    let node = node_entry.deref();

                    let node_id = node.get_id();

                    if !already_called.contains(node_id) {
                        node.handle_value_change(context);
                        already_called.insert(node_id.clone());
                    }
                }
            }
        }
    }

    pub fn register(&self,
                    abort_handle: AbortHandle,
                    node: &Arc<ReactiveConditionInnerNode>) -> Result<(), ReactiveContextError> {
        let bt_node_id =*node.get_id();

        self.abort_handles.insert(bt_node_id, abort_handle);

        for value_name in node.get_value_names().clone() {
            self.nodes_by_value_names
                .entry(value_name)
                .or_insert(DashSet::new())
                .value()
                .insert(node.clone());
        }

        Result::Ok(())
    }
}

