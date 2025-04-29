use candid::{CandidType, Principal};
use ic_cdk::api::{msg_caller, time};
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

use super::action_value::ActionValue;

pub static DEFAULT_CANISTER_STATUS_FETCH_INTERVAL_SECONDS: u64 = 60 * 60; // 1 hour in seconds

impl_storable_for!(ManagementConfig);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct ManagementConfig {
    pub ledger_canister_id: Option<Principal>,
    pub index_canister_id: Option<Principal>,
    pub governance_canister_id: Principal,
    pub deployer_canister_id: Principal,
    pub snsw_canister_id: Principal,
    pub canister_status_fetch_interval_seconds: u64,
    pub deployed_by: Principal,
    pub is_public: bool,
    pub upgraded_at: u64,
    pub created_at: u64,
}

impl ManagementConfig {
    pub fn new(governance_canister_id: Principal, deployed_by: Principal) -> Self {
        Self {
            ledger_canister_id: None,
            index_canister_id: None,
            governance_canister_id,
            deployer_canister_id: msg_caller(),
            canister_status_fetch_interval_seconds: DEFAULT_CANISTER_STATUS_FETCH_INTERVAL_SECONDS,
            snsw_canister_id: Principal::from_text("qaa6y-5yaaa-aaaaa-aaafa-cai").unwrap(),
            deployed_by,
            is_public: false,
            upgraded_at: time(),
            created_at: time(),
        }
    }

    pub fn set_upgraded_at(&mut self, value: u64) -> (ActionValue, ActionValue) {
        let old_updated_at = self.upgraded_at;
        self.upgraded_at = value;
        (
            ActionValue::Number(old_updated_at),
            ActionValue::Number(value),
        )
    }

    pub fn set_canister_status_fetch_interval(&mut self, value: u64) -> (ActionValue, ActionValue) {
        let old_duration = self.canister_status_fetch_interval_seconds;
        self.canister_status_fetch_interval_seconds = value;
        (
            ActionValue::Number(old_duration),
            ActionValue::Number(value),
        )
    }

    pub fn set_public(&mut self, value: bool) -> (ActionValue, ActionValue) {
        let old_value = self.is_public;
        self.is_public = value;
        (ActionValue::Bool(old_value), ActionValue::Bool(value))
    }

    pub fn set_ledger_canister_id(&mut self, value: Principal) {
        self.ledger_canister_id = Some(value);
    }

    pub fn set_index_canister_id(&mut self, value: Principal) {
        self.index_canister_id = Some(value);
    }
}
