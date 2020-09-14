use crate::app::behavior::node::BTNodeAddress;

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

    pub fn tick(&self) {

    }
}