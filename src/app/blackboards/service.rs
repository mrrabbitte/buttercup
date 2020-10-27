use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use bincode::ErrorKind;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use rocksdb::{DB, Error, Options};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::values::{ValueHolder, ValuesPayload};
use std::ffi::OsString;

pub struct BlackboardService {

    local_blackboards: DashMap<Uuid, Arc<RwLock<DB>>>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum BlackboardError {

    AccessError(String),
    BlackboardOfGivenIdNotFound(Uuid),
    DeserializeError(String),
    DestroyError(String),
    LockPoisonedError,
    SerializeError(String)

}

impl BlackboardService {

    pub fn new(local_blackboard_paths: DashMap<Uuid, Arc<RwLock<DB>>>) -> BlackboardService {
        BlackboardService {
            local_blackboards: local_blackboard_paths
        }
    }

    pub fn destroy(&self,
                   blackboard_id: &Uuid) -> Result<(), BlackboardError> {
        match self.get_path_to_destroy(blackboard_id) {
            Ok(path) => self.do_destroy(path),
            Err(err) => Result::Err(err)
        }
    }

    pub fn get_values(&self,
                      blackboard_id: &Uuid,
                      value_names: &HashSet<String>) -> Result<ValuesPayload, BlackboardError> {
        match self.local_blackboards.get(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(kv) =>
                match kv.value().as_ref().read() {
                    Ok(db) =>
                        self.do_get_values(db, value_names),
                    Err(_) =>
                        Result::Err(BlackboardError::LockPoisonedError)
                }
        }
    }

    pub fn put_values(&self,
                      blackboard_id: &Uuid,
                      payload: &ValuesPayload) -> Result<(), BlackboardError> {
        match self.local_blackboards.get(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(kv) =>
                match kv.value().as_ref().write() {
                    Ok(db) => self.do_put_values(db, payload),
                    Err(_) => Result::Err(BlackboardError::LockPoisonedError)
                }
        }
    }

    #[inline(always)]
    fn do_destroy(&self,
                  path: OsString) -> Result<(), BlackboardError> {
        match DB::destroy(&Options::default(), path) {
            Ok(_) => Result::Ok(()),
            Err(err) => Result::Err(BlackboardError::DestroyError(err.into_string()))
        }
    }

    #[inline(always)]
    fn do_get_values(&self,
                     db: RwLockReadGuard<DB>,
                     value_names: &HashSet<String>) -> Result<ValuesPayload, BlackboardError> {
        let mut ret: HashMap<String, ValueHolder> = HashMap::new();
        for value_name in value_names {
            match db.get(value_name) {
                Ok(Some(value)) =>
                    match bincode::deserialize(value.as_slice()) {
                        Ok(value_holder) =>
                            {
                                ret.insert((*value_name).to_string(), value_holder);
                            },
                        Err(e) =>
                            return Result::Err(
                                BlackboardError::DeserializeError(format!("{}", e)))
                    },
                Ok(None) => {},
                Err(e) =>
                    return Result::Err(BlackboardError::AccessError(e.into_string())),
            }
        }
        Result::Ok(ValuesPayload::new(ret))
    }

    #[inline(always)]
    fn do_put_values(&self,
                     db: RwLockWriteGuard<DB>,
                     payload: &ValuesPayload) -> Result<(), BlackboardError> {
        for kv in payload.get_values().iter() {
            match bincode::serialize(kv.1) {
                Ok(value) =>
                    match db.put(kv.0, value) {
                        Ok(_) => {}
                        Err(e) =>
                            return Result::Err(BlackboardError::AccessError(e.into_string()))
                    }
                Err(e) =>
                    return Result::Err(
                        BlackboardError::SerializeError(format!("{}", e)))
            }
        }
        Result::Ok(())
    }

    #[inline(always)]
    fn get_path_to_destroy(&self, blackboard_id: &Uuid) -> Result<OsString, BlackboardError> {
        match self.local_blackboards.remove(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(kv) =>
                {
                    match kv.1.as_ref().write() {
                        Ok(db) =>
                            Result::Ok(db.path().to_path_buf().into_os_string()),
                        Err(_) =>
                            Result::Err(BlackboardError::LockPoisonedError)
                    }
                }
        }
    }

}

impl Default for BlackboardService {
    fn default() -> Self {
        BlackboardService::new(DashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use rocksdb::Options;

    use super::*;

    const FIRST_DB_UUID: u128 = 1;
    const SECOND_DB_UUID: u128 = 2;

    const SOME_KEY: &str = "some_key";
    const OTHER_KEY: &str = "other_key";

    const SOME_VALUE: &str = "some_value";
    const OTHER_VALUE: &str = "other_value";

    lazy_static! {
       static ref SERVICE: BlackboardService = {
         let dbs = DashMap::new();
         dbs.insert(Uuid::from_u128(FIRST_DB_UUID),
                   Arc::new(
                       RwLock::new(
                           DB::open_default(format!("temp_test/{}.rocksdb", FIRST_DB_UUID))
                       .unwrap())));
         dbs.insert(Uuid::from_u128(SECOND_DB_UUID),
                    Arc::new(
                        RwLock::new(
                            DB::open_default(
                                format!("temp_test/{}.rocksdb", SECOND_DB_UUID))
                        .unwrap())));
         BlackboardService::new(dbs)
       };
    }

    #[test]
    fn test_puts_and_gets_values_from_db() {
        let mut values = HashMap::new();
        values.insert(SOME_KEY.to_owned(), ValueHolder::String(SOME_VALUE.to_owned()));
        let payload = ValuesPayload::new(values);
        SERVICE.put_values(
            &Uuid::from_u128(FIRST_DB_UUID), &payload)
            .unwrap();
        let retrieved =
            SERVICE.get_values(&Uuid::from_u128(FIRST_DB_UUID),
                               &HashSet::from_iter(
                                   payload.get_keys().clone()))
                .unwrap();

        assert_eq!(payload, retrieved);
    }

    #[test]
    fn test_puts_and_gets_values_for_different_dbs() {
        let mut values = HashMap::new();
        values.insert(OTHER_KEY.to_owned(), ValueHolder::String(OTHER_VALUE.to_owned()));
        let payload = ValuesPayload::new(values);
        SERVICE.put_values(
            &Uuid::from_u128(FIRST_DB_UUID), &payload)
            .unwrap();
        let retrieved =
            SERVICE.get_values(&Uuid::from_u128(FIRST_DB_UUID),
                               &HashSet::from_iter(
                                   payload.get_keys().clone()))
                .unwrap();

        assert_eq!(payload, retrieved);

        let mut values_for_second = HashMap::new();
        values_for_second.insert(SOME_KEY.to_owned(),
                                 ValueHolder::String(SOME_VALUE.to_owned()));
        let payload_for_second = ValuesPayload::new(values_for_second);
        SERVICE.put_values(
            &Uuid::from_u128(SECOND_DB_UUID), &payload_for_second)
            .unwrap();
        let retrieved_for_second =
            SERVICE.get_values(&Uuid::from_u128(SECOND_DB_UUID),
                               &HashSet::from_iter(
                                   payload_for_second.get_keys().clone()))
                .unwrap();

        assert_eq!(payload_for_second, retrieved_for_second);
        assert_ne!(retrieved, retrieved_for_second);
    }

}