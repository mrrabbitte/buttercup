use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use actix::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::LocalBlackboardError;
use buttercup_variables::VariableValueAccessError;

use crate::context::reactive::ReactiveContextError;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickStatus {

    Success,
    Failure

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum TickError {

    AbortedExecution(i32),
    BlackboardError(i32, LocalBlackboardError),
    CompositeError(i32, Arc<Vec<(i32, TickError)>>),
    ReactiveServiceError(i32, ReactiveContextError),
    VariableValueAccessError(i32, VariableValueAccessError)

}

impl TickError {

    pub fn get_node_id(&self) -> &i32 {
        match self {
            TickError::AbortedExecution(id) => id,
            TickError::BlackboardError(id, _) => id,
            TickError::CompositeError(id, _) => id,
            TickError::ReactiveServiceError(id, _) => id,
            TickError::VariableValueAccessError(id, _) => id
        }
    }

}

#[derive(Default)]
pub struct TickHeader {

    correlation_id: Uuid,

    root_tick_id: Uuid,

    tree_id: i32,
    tree_tick_id: Uuid

}

impl TickHeader {

    pub fn new(correlation_id: Uuid,
               root_tick_id: Uuid,
               tree_id: i32,
               tree_tick_id: Uuid) -> TickHeader {
        TickHeader {
            correlation_id,
            root_tick_id,
            tree_id,
            tree_tick_id
        }
    }

    pub fn get_correlation_id(&self) -> &Uuid {
        &self.correlation_id
    }

    pub fn get_root_tick_id(&self) -> &Uuid {
        &self.root_tick_id
    }

    pub fn get_tree_id(&self) -> &i32 {
        &self.tree_id
    }

    pub fn get_tree_tick_id(&self) -> &Uuid {
        &self.tree_tick_id
    }

    pub fn with_new_root_tick_id(&self,
                                 new_root_tick_id: Uuid) -> TickHeader {
        TickHeader::new(self.correlation_id, new_root_tick_id, self.tree_id, self.tree_tick_id)
    }

}

