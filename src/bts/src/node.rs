use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_blackboards::{LocalBlackboard, LocalBlackboardError};
use buttercup_values::ValuesPayload;

use crate::context::BTNodeExecutionContext;
use crate::events::{BTNodeExecutionEndedEvent, BTNodeExecutionStartedEvent};
use crate::node::action::ActionBTNode;
use crate::node::composite::CompositeBTNode;
use crate::node::decorator::DecoratorBTNode;
use crate::node::decorator::reactive::{DataChangeHandlingError, DataChangeHandlingStatus};
use crate::tick::{TickError, TickHeader, TickStatus};

pub mod action;
pub mod composite;
pub mod decorator;
pub mod root;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum BTNode {

    Action(ActionBTNode),
    Composite(CompositeBTNode),
    Decorator(DecoratorBTNode)

}

#[async_trait]
pub trait BehaviorTreeNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError>;

    fn get_id(&self) -> &i32;

    async fn tick(&self,
                  header: &TickHeader,
                  context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        let node_id = self.get_id();
        let node_tick_id = Uuid::new_v4();

        let started_at = Utc::now().naive_utc();

        context.consume_execution_started_event(
            BTNodeExecutionStartedEvent::new(
                &node_id, &node_tick_id, &started_at, header));

        let result = self.do_tick(header, context).await;

        let ended_at = Utc::now().naive_utc();
        let took_ms = ended_at.signed_duration_since(started_at).num_milliseconds();

        context.consume_execution_ended_event(
            BTNodeExecutionEndedEvent::new(
                &ended_at, &node_id, &node_tick_id, &result, &started_at, header, took_ms
            )
        );

        result
    }

}

#[async_trait]
impl BehaviorTreeNode for BTNode {

    async fn do_tick(&self,
                     header: &TickHeader,
                     context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            BTNode::Action(node) => node.do_tick(header, context).await,
            BTNode::Composite(node) => node.do_tick(header, context).await,
            BTNode::Decorator(node) => node.do_tick(header, context).await,
        }
    }

    fn get_id(&self) -> &i32 {
        match self {
            BTNode::Action(node) => node.get_id(),
            BTNode::Composite(node) => node.get_id(),
            BTNode::Decorator(node) => node.get_id(),
        }
    }
}

impl From<ActionBTNode> for BTNode {
    fn from(node: ActionBTNode) -> Self {
        BTNode::Action(node)
    }
}

impl From<CompositeBTNode> for BTNode {
    fn from(node: CompositeBTNode) -> Self {
        BTNode::Composite(node)
    }
}

impl From<DecoratorBTNode> for BTNode {
    fn from(node: DecoratorBTNode) -> Self {
        BTNode::Decorator(node)
    }
}
