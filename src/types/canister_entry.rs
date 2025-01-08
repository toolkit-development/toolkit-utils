use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::{impl_storable_for, misc::generic::Time};

use super::version::Version;

impl_storable_for!(CanisterEntry);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, Default)]
pub struct CanisterEntry {
    pub version: Option<Version>,
    pub wasm: Vec<u8>,
    pub created_at: Time,
    pub updated_at: Time,
}

impl CanisterEntry {
    pub fn new() -> Self {
        Self {
            version: None,
            wasm: vec![],
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn to_response(&self, canister_id: Principal) -> CanisterEntryResponse {
        CanisterEntryResponse {
            canister_id,
            version: self.version.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct CanisterEntryResponse {
    pub version: Option<Version>,
    pub canister_id: Principal,
    pub created_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NewCanisterArgs {
    pub icp_e8s: Option<u64>,
    pub wasm: Option<Vec<u8>>,
    pub args: Option<Vec<u8>>,
}
