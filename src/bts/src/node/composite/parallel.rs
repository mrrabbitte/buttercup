use std::future::Future;

use actix_rt::Arbiter;
use async_trait::async_trait;
use futures::future::select_all;

use crate::context::BTNodeExecutionContext;
use crate::node::{BehaviorTreeNode, BTNode};
use crate::node::composite::CompositeBTNode;
use crate::tick::{TickError, TickStatus};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ParallelCompositeNode {

    id: i32,
    children: Vec<BTNode>,
    num_successes_to_succeed: usize,
    num_failures_to_fail: usize

}

impl ParallelCompositeNode {

    pub fn new(id: i32,
               children: Vec<BTNode>,
               num_successes_to_succeed: usize) -> Result<ParallelCompositeNode, ()> {
        if num_successes_to_succeed > children.len() {
            return Result::Err(());
        }
        let num_failures_to_fail = children.len() - num_successes_to_succeed + 1;
        Result::Ok(
            ParallelCompositeNode {
                id,
                children,
                num_successes_to_succeed,
                num_failures_to_fail
            }
        )
    }

}

#[async_trait]
impl BehaviorTreeNode for ParallelCompositeNode {
    async fn tick(&self, context: &BTNodeExecutionContext) -> Result<TickStatus, TickError> {
        let mut futures = Vec::new();

        for child in &self.children {
            futures.push(child.tick(context));
        }

        let mut num_failures: usize = 0;
        let mut num_successes: usize = 0;

        let mut errors: Vec<(i32, TickError)> = Vec::new();

        while !futures.is_empty() {
            match select_all(futures).await {
                (Ok(status), _, remaining) => {
                    match status {
                        TickStatus::Success => {
                            num_successes += 1;
                        },
                        TickStatus::Failure => {
                            num_failures += 1;
                        }
                    }
                    futures = remaining;
                }
                (Err(err), _, remaining) => {
                    errors.push((*err.get_node_id(), err));
                    num_failures += 1;
                    futures = remaining;
                }
            }

            if num_successes >= self.num_successes_to_succeed {
                return Result::Ok(TickStatus::Success);
            }

            if num_failures >= self.num_failures_to_fail {
                if errors.is_empty() {
                    return Result::Ok(TickStatus::Failure);
                }
                return Result::Err(TickError::CompositeError(self.id, Arc::new(errors)));
            }
        }
        Result::Ok(TickStatus::Success)
    }
}

impl From<ParallelCompositeNode> for BTNode {
    fn from(node: ParallelCompositeNode) -> Self {
        BTNode::Composite(CompositeBTNode::Parallel(node))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;
    use std::time::Duration;

    use actix_web::test;

    use crate::context::test_utils;
    use crate::node::action::logging::PrintLogActionNode;
    use crate::node::action::wait::WaitDurationActionNode;

    use super::*;

    #[actix_rt::test]
    async fn test_finishes_based_on_minimal_number_of_successes() {
        let path = {
            let context = Default::default();
            let children: Vec<BTNode> = vec![
                PrintLogActionNode::new(1, "I am one.".to_string()).into(),
                WaitDurationActionNode::new(2, Duration::from_millis(10)).into(),
                PrintLogActionNode::new(3, "I am two.".to_string()).into(),
                PrintLogActionNode::new(4, "I am four.".to_string()).into()];
            match ParallelCompositeNode::new(5, children, 3)
                .unwrap()
                .tick(&context)
                .await {
                Ok(status) => {
                    assert_eq!(TickStatus::Success, status)
                }
                Err(err) => panic!("Expected success.")
            }

            test_utils::get_path(&context)
        };
        
        test_utils::destroy(path);
    }

}
