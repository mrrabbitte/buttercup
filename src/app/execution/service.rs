use std::future::Future;

use actix::Arbiter;
use futures_channel::oneshot::Canceled;
use rand::seq::SliceRandom;
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

    pub fn exec<F, R>(&self, f: F)
                      -> Result<impl Future<Output = Result<R, Canceled>>, ExecutionServiceError>
        where
            F: FnOnce() -> R + Send + 'static,
            R: Send + 'static,
    {
        match self.arbiters.choose(&mut thread_rng()) {
            None => Result::Err(ExecutionServiceError::GotEmptyArbitersCollection),
            Some(arbiter) => Result::Ok(arbiter.exec(f))
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::task::Poll;

    #[actix_rt::test]
    async fn test_execution_service_builds_correctly() {
        assert_eq!(ExecutionService::new(vec![]).is_err(), true);
        assert_eq!(ExecutionService::new(vec![Arbiter::new()]).is_ok(), true);
    }

    #[actix_rt::test]
    async fn test_execution_service_schedules_task() {
        let expected = 43200;
        match ExecutionService::new(vec![Arbiter::new()]) {
            Ok(service) => {
                let result =
                    service.exec(move || expected as i32 + 10);
                assert_eq!(result.is_ok(), true);
                match result {
                    Ok(future) => {
                        assert_eq!(expected + 10, future.await.unwrap());
                    },
                    Err(_) => panic!("Got error spawning future!")
                }
            },
            Err(_) => panic!("Got error building service!")
        }
    }
}