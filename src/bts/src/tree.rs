use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::root::RootBTNode;
use crate::tick::{TickError, TickStatus};

pub struct BehaviorTree {

    id: i32,
    root: RootBTNode

}

impl BehaviorTree {

    pub fn new(id: i32,
               root: RootBTNode) -> BehaviorTree {
        BehaviorTree {
            id,
            root
        }
    }

    pub async fn tick(&self,
                      context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        self.root.tick(context).await
    }

}

#[derive(Default)]
pub struct BehaviorTreeService {

    trees: DashMap<i32, Arc<BehaviorTree>>

}

impl BehaviorTreeService {

    pub fn insert(&self,
                  tree: BehaviorTree) {
        self.trees.insert(tree.id, Arc::new(tree));
    }

    pub fn get_by_id(&self,
                     id: &i32) -> Option<Arc<BehaviorTree>> {
        self.trees.get(id).map(|tree_arc| tree_arc.clone())
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;
    use uuid::Uuid;

    use buttercup_blackboards::LocalBlackboard;

    use crate::context::test_utils;
    use crate::node::action::logging::PrintLogActionNode;
    use crate::node::root::one_off::OneOffRootBTNode;

    use super::*;

    #[actix_rt::test]
    async fn test_returns_status() {
        let path = {
            let context = Default::default();
            assert_eq!(Result::Ok(TickStatus::Success),
                       BehaviorTree::new(1,
                                         OneOffRootBTNode::new(
                                             1,
                                                               PrintLogActionNode::new(
                                             1, "hello".to_owned())
                                             .into()).into())
                           .tick(&context).await);

            test_utils::get_path(&context)
        };

        test_utils::destroy(path);
    }

}
