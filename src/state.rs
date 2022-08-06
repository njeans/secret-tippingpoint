use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::vec::Vec;
use std::collections::HashMap;
use cosmwasm_std::{CanonicalAddr, Storage, ReadonlyStorage, StdResult, StdError, HumanAddr};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::serialization::{Bincode2, Serde};
use std::{any::type_name};


pub static CONFIG_KEY: &[u8] = b"config";

pub static CONFIG_KEY_M: &[u8] = b"manufacturers";
pub static CONFIG_KEY_P: &[u8] = b"pharmacists";
pub static CONFIG_KEY_B: &[u8] = b"batches";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: CanonicalAddr,
}

pub type ManufactureId = HumanAddr;
pub type PharmacistId = HumanAddr;
pub type SymptomToken = u64;
pub type BatchId = u64;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchState {
    pub locations: Vec<String>,
    pub threshold: u64,
    pub count: u64,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}


// pub fn GetFullKey<T>(base: [u8], key: &T) -> Vec<u8> {
//     let keySerialized = key.to_be_bytes();
//     let fullKey: Vec<u8> = [base, &keySerialized].concat();
//     fullKey
// }



// Save will save if it is not a new value
//
pub fn update<T:DeserializeOwned+Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    let result: Option<T> = may_load(&*storage, key).ok().unwrap();
    match result {
        Some(_) => {
            storage.set(key, &Bincode2::serialize(&value)?);
            Ok(())
        }
        None => {
            Err(StdError::Unauthorized{backtrace: None})
        }
    }
}

pub fn register<T: DeserializeOwned+Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    let result: Option<T> = may_load(&*storage, key).ok().unwrap();
    match result {
        Some(_) => {
            Err(StdError::Unauthorized{backtrace: None})
        }
        None => {
            storage.set(key, &Bincode2::serialize(&value)?);
            Ok(())
        }
    }
}

pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Ok(Some(Bincode2::deserialize(&value).ok().unwrap())),
        None => Ok(None),
    }
}
