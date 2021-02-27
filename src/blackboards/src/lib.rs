#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use dashmap::DashMap;
use rocksdb::{DB, Error, Options};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_values::{ValueHolder, ValuesPayload};

#[derive(Default)]
pub struct LocalBlackboardService {

    local_blackboards: DashMap<Uuid, Arc<LocalBlackboard>>

}

impl LocalBlackboardService {

    pub fn new(local_blackboards: DashMap<Uuid, Arc<LocalBlackboard>>) -> LocalBlackboardService {
        LocalBlackboardService {
            local_blackboards
        }
    }

    pub fn create(&self,
                  blackboard_id: Uuid,
                  path: OsString) -> Result<(), LocalBlackboardError> {
        self.local_blackboards.insert(blackboard_id,
                                      Arc::new(LocalBlackboard::new(path)?));

        Result::Ok(())
    }

    ///
    /// Note that this may delete bb but will not cleanup afterwards, it has to do with locking
    /// scheme of RocksDb. This will be handled in self-healing feature.
    ///
    pub fn destroy(&self,
                   blackboard_id: &Uuid) -> Result<(), LocalBlackboardError> {

        let path = {
            match self.local_blackboards.remove(blackboard_id) {
                Some((_, blackboard)) =>
                    blackboard.as_ref().get_path(),
                None =>
                    Result::Err(
                        LocalBlackboardError::BlackboardOfGivenIdNotFound(blackboard_id.clone()))
            }
        }?;

        LocalBlackboard::destroy(path)
    }

    pub fn get(&self, blackboard_id: &Uuid) -> Result<Arc<LocalBlackboard>, LocalBlackboardError> {
        self.local_blackboards
            .get(blackboard_id)
            .map_or_else(
                || Result::Err(
                    LocalBlackboardError::BlackboardOfGivenIdNotFound(blackboard_id.clone())),
                |kv| Result::Ok(kv.value().clone()))

    }

    pub fn is_empty(&self) -> bool {
        self.local_blackboards.is_empty()
    }

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum LocalBlackboardError {

    AccessError(String),
    BlackboardOfGivenIdNotFound(Uuid),
    DbError(String),
    DeserializeError(String),
    LockPoisonedError(String),
    SerializeError(String)

}

impl From<PoisonError<RwLockReadGuard<'_, DB>>> for LocalBlackboardError {
    fn from(err: PoisonError<RwLockReadGuard<'_, DB>>) -> Self {
        LocalBlackboardError::LockPoisonedError(err.to_string())
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, DB>>> for LocalBlackboardError {
    fn from(err: PoisonError<RwLockWriteGuard<'_, DB>>) -> Self {
        LocalBlackboardError::LockPoisonedError(err.to_string())
    }
}

impl From<rocksdb::Error> for LocalBlackboardError {
    fn from(err: Error) -> Self {
        LocalBlackboardError::DbError(err.into_string())
    }
}

pub struct LocalBlackboard {

    db: Arc<RwLock<DB>>

}

impl LocalBlackboard {

    pub fn new(path: OsString) -> Result<LocalBlackboard, LocalBlackboardError>  {
        Result::Ok(
            LocalBlackboard {
                db: Arc::new(RwLock::new(DB::open_default(path)?))
            }
        )
    }

    pub fn destroy(path: OsString) -> Result<(), LocalBlackboardError> {
        DB::destroy(
            &Options::default(),
            path)?;

        Result::Ok(())
    }

    pub fn get_path(&self) -> Result<OsString, LocalBlackboardError> {
        Result::Ok(self.db.as_ref().read()?.path().to_path_buf().into_os_string())
    }

    pub fn get_value(&self,
                     value_name: &String) -> Result<Option<ValueHolder>, LocalBlackboardError> {
        LocalBlackboard::do_get_value(&self.db.as_ref().read()?, value_name)
    }

    pub fn get_values(&self,
                      value_names: &HashSet<String>) -> Result<ValuesPayload, LocalBlackboardError> {
        LocalBlackboard::do_get_values(self.db.as_ref().read()?, value_names)
    }

    pub fn put_values(&self,
                      payload: &ValuesPayload) -> Result<(), LocalBlackboardError> {
        LocalBlackboard::do_put_values(self.db.as_ref().write()?, payload)
    }

    #[inline(always)]
    fn do_get_values(db: RwLockReadGuard<DB>,
                     value_names: &HashSet<String>) -> Result<ValuesPayload, LocalBlackboardError> {
        let mut ret: HashMap<String, ValueHolder> = HashMap::new();
        for value_name in value_names {
            match LocalBlackboard::do_get_value(&db, value_name) {
                Ok(value_holder_opt) =>
                    match value_holder_opt {
                        None => {},
                        Some(value_holder) => {
                            ret.insert((*value_name).to_string(), value_holder);
                        }
                    },
                Err(err) => return Result::Err(err)
            }
        }
        Result::Ok(ValuesPayload::new(ret))
    }

    #[inline(always)]
    fn do_get_value(db: &RwLockReadGuard<DB>,
                    value_name: &String) -> Result<Option<ValueHolder>, LocalBlackboardError> {
        match db.get(value_name) {
            Ok(Some(value)) =>
                match bincode::deserialize(value.as_slice()) {
                    Ok(value_holder) => Result::Ok(Option::Some(value_holder)),
                    Err(e) =>
                        Result::Err(
                            LocalBlackboardError::DeserializeError(format!("{}", e)))
                },
            Ok(None) => Result::Ok(Option::None),
            Err(e) =>
                Result::Err(
                    LocalBlackboardError::AccessError(e.into_string())),
        }
    }
    #[inline(always)]
    fn do_put_values(db: RwLockWriteGuard<DB>,
                     payload: &ValuesPayload) -> Result<(), LocalBlackboardError> {
        for kv in payload.get_values().iter() {
            match bincode::serialize(kv.1) {
                Ok(value) =>
                    match db.put(kv.0, value) {
                        Ok(_) => {}
                        Err(e) =>
                            return Result::Err(LocalBlackboardError::AccessError(e.into_string()))
                    }
                Err(e) =>
                    return Result::Err(
                        LocalBlackboardError::SerializeError(format!("{}", e)))
            }
        }
        Result::Ok(())
    }

}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::iter::FromIterator;

    use super::*;

    const FIRST_DB_UUID: u128 = 1;
    const SECOND_DB_UUID: u128 = 2;
    const THIRD_DB_UUID: u128 = 3;

    const SOME_KEY: &str = "some_key";
    const OTHER_KEY: &str = "other_key";

    const SOME_VALUE: &str = "some_value";
    const OTHER_VALUE: &str = "other_value";

    const TEMP_TEST: &str = "temp_test/";

    lazy_static! {
       static ref SERVICE: LocalBlackboardService = {
         let service = LocalBlackboardService::default();

         service.create(
                    Uuid::from_u128(FIRST_DB_UUID),
                    format!("temp_test/{}.rocksdb", FIRST_DB_UUID).into()).unwrap();
         service.create(
                    Uuid::from_u128(SECOND_DB_UUID),
                    format!("temp_test/{}.rocksdb", SECOND_DB_UUID).into()).unwrap();
         service.create(
                    Uuid::from_u128(THIRD_DB_UUID),
                    format!("temp_test/{}.rocksdb", THIRD_DB_UUID).into()).unwrap();

         service
       };
    }

    #[test]
    fn test_puts_and_gets_values_from_db() {
        {
            let mut values = HashMap::new();

            values.insert(SOME_KEY.to_owned(), ValueHolder::String(
                Arc::new(SOME_VALUE.to_owned())));

            let first_bb =
                SERVICE.get(&Uuid::from_u128(FIRST_DB_UUID)).unwrap();

            let payload = ValuesPayload::new(values);
            first_bb.put_values(&payload)
                .unwrap();
            let retrieved =
                first_bb.get_values(&HashSet::from_iter(
                    payload.get_keys().clone()))
                    .unwrap();

            assert_eq!(payload, retrieved);
        }

        cleanup(FIRST_DB_UUID);
    }

    #[test]
    fn test_puts_and_gets_values_for_different_dbs() {
        {
            let mut values = HashMap::new();

            values.insert(OTHER_KEY.to_owned(), ValueHolder::String(
                Arc::new(OTHER_VALUE.to_owned())));
            let payload = ValuesPayload::new(values);

            let second_bb =
                SERVICE.get(&Uuid::from_u128(SECOND_DB_UUID)).unwrap();
            second_bb.put_values(&payload).unwrap();

            let retrieved = second_bb.get_values(
                &HashSet::from_iter(payload.get_keys().clone())).unwrap();

            assert_eq!(payload, retrieved);

            let mut values_for_second = HashMap::new();
            values_for_second.insert(SOME_KEY.to_owned(),
                                     ValueHolder::String(Arc::new(SOME_VALUE.to_owned())));
            let payload_for_second = ValuesPayload::new(values_for_second);

            let third_bb =
                SERVICE.get(&Uuid::from_u128(THIRD_DB_UUID)).unwrap();
            third_bb.put_values(&payload_for_second).unwrap();

            let retrieved_for_second =
                third_bb.get_values(
                    &HashSet::from_iter(payload_for_second.get_keys().clone()))
                    .unwrap();

            assert_eq!(payload_for_second, retrieved_for_second);
            assert_ne!(retrieved, retrieved_for_second);
        }

        cleanup(SECOND_DB_UUID);
        cleanup(THIRD_DB_UUID);
    }

    fn cleanup(uuid: u128) {
        SERVICE.destroy(&Uuid::from_u128(uuid)).unwrap();
        if SERVICE.is_empty() {
            match remove_dir_all(TEMP_TEST) {
                Ok(_) => {}
                Err(err) => println!("{}", err)
            }
        }
    }

}