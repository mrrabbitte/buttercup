use crate::app::behavior::node::{BTNodeAddress, BTNode};
use crate::app::behavior::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    nodes: Vec<BTNode>,
    current_node: BTNodeAddress

}

impl BehaviorTree {

    pub fn new(id: i32,
               nodes: Vec<BTNode>,
               root_node: BTNodeAddress) -> BehaviorTree {
        BehaviorTree {
            id,
            nodes,
            current_node: root_node
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
