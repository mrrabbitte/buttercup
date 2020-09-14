use crate::app::behavior::node::BTNodeAddress;
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    root_node: BTNodeAddress

}

impl BehaviorTree {

    pub fn new(id: i32,
               root_node: BTNodeAddress) -> BehaviorTree {
        BehaviorTree {
            id,
            root_node
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
                   BehaviorTree::new(1, BTNodeAddress::new(1, 1)).tick())
    }

}
