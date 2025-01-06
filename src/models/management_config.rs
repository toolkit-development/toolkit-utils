use candid::{CandidType, Principal};
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
    pub canister_status_fetch_interval_seconds: u64,
    pub deployed_by: Principal,
}

impl ManagementConfig {
    pub fn new(governance_canister_id: Principal, deployed_by: Principal) -> Self {
        Self {
            ledger_canister_id: None,
            index_canister_id: None,
            governance_canister_id,
            canister_status_fetch_interval_seconds: DEFAULT_CANISTER_STATUS_FETCH_INTERVAL_SECONDS,
            deployed_by,
        }
    }

    pub fn set_canister_status_fetch_interval(&mut self, value: u64) -> (ActionValue, ActionValue) {
        let old_duration = self.canister_status_fetch_interval_seconds;
        self.canister_status_fetch_interval_seconds = value;
        (
            ActionValue::Number(old_duration),
            ActionValue::Number(value),
        )
    }
}
