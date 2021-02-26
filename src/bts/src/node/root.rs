use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::BehaviorTreeNode;
use crate::node::root::one_off::OneOffRootBTNode;
use crate::node::root::reactive::ReactiveRootBTNode;
use crate::node::root::to_first_fail::ToFirstFailRootBTNode;
use crate::node::root::until_stopped::UntilStoppedRootBTNode;
use crate::tick::{TickError, TickStatus};

pub mod to_first_fail;
pub mod one_off;
pub mod reactive;
pub mod until_stopped;

pub enum RootNode {

    OneOff(OneOffRootBTNode),
    Reactive(ReactiveRootBTNode),
    ToFirstFail(ToFirstFailRootBTNode),
    UntilStopped(UntilStoppedRootBTNode),

}

#[async_trait(?Send)]
impl BehaviorTreeNode for RootNode {
    async fn tick(&self,
                  context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            RootNode::OneOff(node) => node.tick(context).await,
            RootNode::Reactive(node) => node.tick(context).await,
            RootNode::ToFirstFail(node) => node.tick(context).await,
            RootNode::UntilStopped(node) => node.tick(context).await
        }
    }
}

