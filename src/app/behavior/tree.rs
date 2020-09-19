use crate::app::behavior::node::{BTNodeAddress, BTNode, BTNodeExecutionContext};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    context: BTNodeExecutionContext

}

impl BehaviorTree {

    pub fn new(id: i32) -> BehaviorTree {
        BehaviorTree {
            id,
            context: BTNodeExecutionContext::new()
        }
    }

    pub fn tick(&self) -> Result<TickStatus, TickError> {
        Result::Ok(TickStatus::Success)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::app::address::Address;
    use crate::app::behavior::node::BTNodeAddress;

    #[test]
    fn test_returns_status() {
        assert_eq!(Result::Ok(TickStatus::Success),
                   BehaviorTree::new(1).tick())
    }

}
