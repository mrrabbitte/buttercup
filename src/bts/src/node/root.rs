use async_trait::async_trait;

use crate::context::BTNodeExecutionContext;
use crate::node::BehaviorTreeNode;
use crate::node::root::one_off::OneOffRootBTNode;
use crate::node::root::reactive::ReactiveRootBTNode;
use crate::node::root::to_first::{ToFirstErrorRootBTNode, ToFirstFailureRootBTNode};
use crate::node::root::until_stopped::UntilStoppedRootBTNode;
use crate::tick::{TickError, TickStatus};

pub mod to_first;
pub mod one_off;
pub mod reactive;
pub mod until_stopped;

pub enum RootBTNode {

    OneOff(OneOffRootBTNode),
    Reactive(ReactiveRootBTNode),
    ToFirstError(ToFirstErrorRootBTNode),
    ToFirstFailure(ToFirstFailureRootBTNode),
    UntilStopped(UntilStoppedRootBTNode),

}

#[async_trait]
impl BehaviorTreeNode for RootBTNode {
    async fn tick(&self,
                  context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        match self {
            RootBTNode::OneOff(node) => node.tick(context).await,
            RootBTNode::Reactive(node) => node.tick(context).await,
            RootBTNode::ToFirstError(node) => node.tick(context).await,
            RootBTNode::ToFirstFailure(node) => node.tick(context).await,
            RootBTNode::UntilStopped(node) => node.tick(context).await
        }
    }
}

impl RootBTNode {
    pub fn can_be_subtree_root(&self) -> bool {
        match self {
            RootBTNode::OneOff(_) => true,
            _ => false
        }
    }
}

impl From<OneOffRootBTNode> for RootBTNode {
    fn from(node: OneOffRootBTNode) -> Self {
        RootBTNode::OneOff(node)
    }
}

impl From<ReactiveRootBTNode> for RootBTNode {
    fn from(node: ReactiveRootBTNode) -> Self {
        RootBTNode::Reactive(node)
    }
}

impl From<ToFirstErrorRootBTNode> for RootBTNode {
    fn from(node: ToFirstErrorRootBTNode) -> Self {
        RootBTNode::ToFirstError(node)
    }
}

impl From<ToFirstFailureRootBTNode> for RootBTNode {
    fn from(node: ToFirstFailureRootBTNode) -> Self {
        RootBTNode::ToFirstFailure(node)
    }
}

impl From<UntilStoppedRootBTNode> for RootBTNode {
    fn from(node: UntilStoppedRootBTNode) -> Self {
        RootBTNode::UntilStopped(node)
    }
}
