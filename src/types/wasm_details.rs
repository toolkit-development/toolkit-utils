use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::{
    impl_storable_for,
    misc::{generic::Time, hash::generate_checksum},
};

use super::version::Version;

impl_storable_for!(WasmDetails);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, Default)]
pub struct WasmDetails {
    pub version: Version,
    pub wasm_hash: Vec<u8>,
    pub created_at: Time,
}

impl WasmDetails {
    pub fn new(wasm: Vec<u8>, version: Version) -> Self {
        Self {
            wasm_hash: generate_checksum(&wasm),
            created_at: time(),
            version,
        }
    }
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct AllWasmDetails {
    pub management: WasmDetails,
    pub governance: WasmDetails,
}
