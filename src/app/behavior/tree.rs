use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::{BTNode, BTNodeAddress};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    context: BTNodeExecutionContext

}

impl BehaviorTree {

    pub fn new(id: i32,
               context: BTNodeExecutionContext) -> BehaviorTree {
        BehaviorTree {
            id,
            context
        }
    }

    pub fn tick(&self) -> Result<TickStatus, TickError> {
        Result::Ok(TickStatus::Success)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;
    use uuid::Uuid;

    use crate::app::address::Address;
    use crate::app::behavior::node::BTNodeAddress;
    use crate::app::blackboards::service::BlackboardService;

    use super::*;

    #[test]
    fn test_returns_status() {
        assert_eq!(Result::Ok(TickStatus::Success),
                   BehaviorTree::new(1,
                                     BTNodeExecutionContext::new(
                                         Uuid::from_u128(1),
                                         Arc::new(
                                             BlackboardService::new(
                                                 DashMap::new()))))
                       .tick())
    }

}
