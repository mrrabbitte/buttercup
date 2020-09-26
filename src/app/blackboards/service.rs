use std::collections::HashMap;

use bincode::ErrorKind;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use rocksdb::{DB, Error, Options};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::values::{ValueHolder, ValuesPayload};

/// Note: There is no notion of transaction and values are
/// put and get up to the first failure, hence any behavior
/// that needs such guarantees should be based on Agents
/// polling for data.
pub struct BlackboardService {

    local_blackboards: DashMap<Uuid, DB>

}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum BlackboardError {

    AccessError(String),
    BlackboardOfGivenIdNotFound(Uuid),
    DeserializeError(String),
    DestroyError(String),
    SerializeError(String)

}

impl BlackboardService {

    pub fn new(local_blackboards: DashMap<Uuid, DB>) -> BlackboardService {
        BlackboardService {
            local_blackboards
        }
    }

    pub fn get_values(&self,
                      blackboard_id: &Uuid,
                      value_names: &Vec<&String>) -> Result<ValuesPayload, BlackboardError> {
        match self.local_blackboards.get(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(db) => self.do_get_values(db.value(), value_names)
        }
    }

    pub fn destroy(&self,
                   blackboard_id: &Uuid) -> Result<(), BlackboardError> {
        match self.local_blackboards.remove(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(kv) => {
                match DB::destroy(&Options::default(), kv.1.path()) {
                    Ok(_) => Result::Ok(()),
                    Err(err) =>
                        Result::Err(BlackboardError::DestroyError(err.into_string()))
                }
            }
        }
    }

    pub fn put_values(&self,
                      blackboard_id: &Uuid,
                      payload: &ValuesPayload) -> Result<(), BlackboardError> {
        match self.local_blackboards.get(blackboard_id) {
            None => Result::Err(
                BlackboardError::BlackboardOfGivenIdNotFound(*blackboard_id)),
            Some(db) => self.do_put_values(db.value(), payload)
        }
    }

    #[inline(always)]
    fn do_get_values(&self,
                     db: &DB,
                     value_names: &Vec<&String>) -> Result<ValuesPayload, BlackboardError> {
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
                     db: &DB,
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

}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use rocksdb::Options;

    use super::*;

    const FIRST_DB_UUID: u128 = 1;
    const SECOND_DB_UUID: u128 = 1;

    #[test]
    fn test_puts_and_gets_values_from_db() {
        let service = build_service();
        let mut values = HashMap::new();
        values.insert("something".to_string(), ValueHolder::String("dsfs".to_string()));
        let payload = ValuesPayload::new(values);
        let value_names = Vec::from_iter(payload.get_values().keys().into_iter());
        service.put_values(
            &Uuid::from_u128(FIRST_DB_UUID),
            &payload);
        let retrieved =
            service.get_values(&Uuid::from_u128(FIRST_DB_UUID),
                           &Vec::from_iter(payload.get_values().keys().into_iter()));

        assert_eq!(payload, retrieved.unwrap());

        service.destroy(&Uuid::from_u128(FIRST_DB_UUID)).unwrap();
    }

    // #[test]
    // fn test_puts_and_gets_values_for_different_dbs() {
    //     let service = build_service();
    //
    //     cleanup();
    // }

    fn build_service() -> BlackboardService {
        let mut dbs = DashMap::new();
        dbs.insert(Uuid::from_u128(FIRST_DB_UUID),
                   DB::open_default(format!("test_temp/{}.rocksdb", FIRST_DB_UUID)).unwrap());
        // dbs.insert(Uuid::from_u128(SECOND_DB_UUID),
        //            DB::open_default(format!("temp/{}.rocksdb", SECOND_DB_UUID)).unwrap());
        BlackboardService::new(dbs)
    }

}