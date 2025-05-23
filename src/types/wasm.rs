use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::{
    impl_storable_for,
    misc::{generic::Time, hash::generate_checksum},
};

use super::{version::Version, wasm_details::WasmDetails};

impl_storable_for!(Wasm);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, Default)]
pub struct Wasm {
    pub version: Version,
    pub wasm: Vec<u8>,
    pub created_at: Time,
}

impl Wasm {
    pub fn new(wasm: Vec<u8>, version: Version) -> Self {
        Self {
            wasm,
            created_at: time(),
            version,
        }
    }

    pub fn to_details(&self) -> WasmDetails {
        WasmDetails {
            version: self.version.clone(),
            wasm_hash: generate_checksum(&self.wasm),
            created_at: self.created_at,
        }
    }
}
