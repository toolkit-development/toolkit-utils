use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(ProjectRegistryEntry);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct ProjectRegistryEntry {
    pub governance_canister_id: Principal,
    pub name: String,
    pub description: String,
    pub url: String,
    pub is_public: bool,
    pub updated_at: u64,
    pub created_at: u64,
}

impl From<PostProjectRegistryEntry> for ProjectRegistryEntry {
    fn from(post_registry_entry: PostProjectRegistryEntry) -> Self {
        Self {
            governance_canister_id: post_registry_entry.governance_canister_id,
            name: post_registry_entry.name,
            description: post_registry_entry.description,
            url: post_registry_entry.url,
            is_public: post_registry_entry.is_public,
            updated_at: time(),
            created_at: time(),
        }
    }
}

impl ProjectRegistryEntry {
    pub fn to_response(&self, management_canister_id: Principal) -> ProjectRegistryEntryResponse {
        ProjectRegistryEntryResponse {
            management_canister_id,
            governance_canister_id: self.governance_canister_id,
            name: self.name.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            is_public: self.is_public,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct PostProjectRegistryEntry {
    pub governance_canister_id: Principal,
    pub name: String,
    pub description: String,
    pub url: String,
    pub is_public: bool,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct ProjectRegistryEntryResponse {
    pub management_canister_id: Principal,
    pub governance_canister_id: Principal,
    pub name: String,
    pub description: String,
    pub url: String,
    pub is_public: bool,
    pub updated_at: u64,
    pub created_at: u64,
}
