use std::sync::Arc;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    context: Arc<BTNodeExecutionContext>,
    root: BTNode
}

impl BehaviorTree {

    pub fn new(id: i32,
               context: Arc<BTNodeExecutionContext>,
               root: BTNode) -> BehaviorTree {
        BehaviorTree {
            id,
            context,
            root
        }
    }

    pub async fn tick(&self) -> Result<TickStatus, TickError> {
        self.root.tick(self.context.as_ref()).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;
    use uuid::Uuid;

    use crate::node::action::logging::PrintLogActionNode;
    use buttercup_blackboards::BlackboardService;

    use super::*;

    #[actix_rt::test]
    async fn test_returns_status() {
        assert_eq!(Result::Ok(TickStatus::Success),
                   BehaviorTree::new(1,
                                     Arc::new(Default::default()),
                                     PrintLogActionNode::new(
                                         1, "hello".to_owned())
                                         .into())
                       .tick().await)
    }

}
