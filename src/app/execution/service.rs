use actix::{Arbiter};
use rand::seq::SliceRandom;
use std::future::Future;
use futures_channel::oneshot::Canceled;
use rand::thread_rng;

pub struct ExecutionService {

    arbiters: Vec<Arbiter>

}

pub enum ExecutionServiceError {

    GotEmptyArbitersCollection

}

impl ExecutionService {

    pub fn new(arbiters: Vec<Arbiter>) -> Result<ExecutionService, ExecutionServiceError>  {
        if arbiters.is_empty() {
            return Result::Err(ExecutionServiceError::GotEmptyArbitersCollection);
        }
        Result::Ok(
            ExecutionService {
                arbiters
            }
        )
    }

    pub fn exec<F, R>(&self, f: F) -> impl Future<Output = Result<R, Canceled>>
        where
            F: FnOnce() -> R + Send + 'static,
            R: Send + 'static,
    {
        match self.arbiters.choose(&mut thread_rng()) {
            None => panic!("Arbiters list should be never empty."),
            Some(arbiter) => arbiter.exec(f)
        }
    }

}