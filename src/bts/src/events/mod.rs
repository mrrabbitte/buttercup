use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use crate::tick::{TickError, TickHeader, TickStatus};

#[derive(Debug)]
pub struct BTNodeExecutionEndedEvent<'e> {

    id: Uuid,
    correlation_id: &'e Uuid,
    created_at: NaiveDateTime,

    ended_at: &'e NaiveDateTime,

    node_id: &'e i32,
    node_tick_id: &'e Uuid,

    result: &'e Result<TickStatus, TickError>,

    root_tick_id: &'e Uuid,

    started_at: &'e NaiveDateTime,

    took_ms: i64,

    tree_id: &'e i32,
    tree_tick_id: &'e Uuid,

}

impl<'e> BTNodeExecutionEndedEvent<'e> {

    pub fn new(ended_at: &'e NaiveDateTime,
               node_id: &'e i32,
               node_tick_id: &'e Uuid,
               result: &'e Result<TickStatus, TickError>,
               started_at: &'e NaiveDateTime,
               tick_header: &'e TickHeader,
               took_ms: i64) -> BTNodeExecutionEndedEvent<'e> {
        BTNodeExecutionEndedEvent {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            correlation_id: tick_header.get_correlation_id(),
            node_id,
            node_tick_id,
            result,
            root_tick_id: tick_header.get_root_tick_id(),
            started_at,
            took_ms,
            tree_id: tick_header.get_tree_id(),
            tree_tick_id: tick_header.get_tree_tick_id(),
            ended_at
        }
    }

}

#[derive(Debug)]
pub struct BTNodeExecutionStartedEvent<'e> {

    id: Uuid,
    correlation_id: &'e Uuid,
    created_at: NaiveDateTime,

    node_id: &'e i32,
    node_tick_id: &'e Uuid,

    root_tick_id: &'e Uuid,

    started_at: &'e NaiveDateTime,

    tree_id: &'e i32,
    tree_tick_id: &'e Uuid,

}

impl<'e> BTNodeExecutionStartedEvent<'e> {

    pub fn new(node_id: &'e i32,
               node_tick_id: &'e Uuid,
               started_at: &'e NaiveDateTime,
               tick_header: &'e TickHeader) -> BTNodeExecutionStartedEvent<'e> {
        BTNodeExecutionStartedEvent {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            correlation_id: tick_header.get_correlation_id(),
            node_id,
            node_tick_id,
            root_tick_id: tick_header.get_root_tick_id(),
            started_at,
            tree_id: tick_header.get_tree_id(),
            tree_tick_id: tick_header.get_tree_tick_id(),
        }
    }

}