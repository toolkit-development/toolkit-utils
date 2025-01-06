use candid::{CandidType, Principal};
use ic_cdk::api::time;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{governance_types::ProposalType, impl_storable_for};

use super::{action_value::ActionValue, governance_types::GovernanceType};

impl_storable_for!(GovernanceConfig);

pub static DEFAULT_PROPOSAL_DURATION_SECONDS: u64 = 60 * 60 * 24 * 2; // 2 days in seconds
pub static PROPOSAL_DURATION_LOWER_LIMIT_SECONDS: u64 = 60 * 60 * 24; // 1 day in seconds
pub static PROPOSAL_DURATION_UPPER_LIMIT_SECONDS: u64 = 60 * 60 * 24 * 4; // 4 days in seconds

pub static CANISTER_STATUS_FETCH_INTERVAL_LOWER_LIMIT_SECONDS: u64 = 60 * 5; // 5 minutes in seconds
pub static CANISTER_STATUS_FETCH_INTERVAL_UPPER_LIMIT_SECONDS: u64 = 60 * 60 * 24 * 365; // 1 year in seconds

pub static DEFAULT_MAX_LOG_ENTRIES: u64 = 1000;

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct GovernanceConfig {
    pub proposal_duration_seconds: u64,
    pub governance_type: Option<String>,
    pub management_canister_id: Principal,
    pub is_public: bool,
    pub owner: Account,
    pub hotkeys_enabled: bool,
    pub max_log_entries: u64,
    pub updated_at: u64,
}

impl GovernanceConfig {
    pub fn new(owner: Principal, is_public: bool, management_canister_id: Principal) -> Self {
        Self {
            is_public,
            hotkeys_enabled: true,
            governance_type: None,
            proposal_duration_seconds: DEFAULT_PROPOSAL_DURATION_SECONDS,
            updated_at: time(),
            max_log_entries: DEFAULT_MAX_LOG_ENTRIES,
            management_canister_id,
            owner: Account::from(owner),
        }
    }

    pub fn set_governance(&mut self, value: GovernanceType) -> (ActionValue, ActionValue) {
        let old_governance = self.governance_type.clone();

        if let GovernanceType::None = value {
            self.governance_type = None
        } else {
            self.governance_type = Some(value.to_string());
        }

        (
            ActionValue::String(old_governance.unwrap_or_default()),
            ActionValue::String(value.to_string()),
        )
    }

    pub fn get_governance(&self) -> GovernanceType {
        self.governance_type
            .as_ref()
            .map(|governance_type| GovernanceType::from_string(governance_type))
            .unwrap_or(GovernanceType::None)
    }

    pub fn governance_is_proposal_based(&self) -> bool {
        matches!(self.get_governance(), GovernanceType::Proposal(_))
    }

    pub fn governance_is_token_based(&self) -> bool {
        matches!(
            self.get_governance(),
            GovernanceType::Proposal(ProposalType::Token(_))
        )
    }

    pub fn set_proposal_duration_seconds(&mut self, value: u64) -> (ActionValue, ActionValue) {
        let old_duration = self.proposal_duration_seconds;
        self.proposal_duration_seconds = value;
        (
            ActionValue::Number(old_duration),
            ActionValue::Number(value),
        )
    }

    pub fn set_max_log_entries(&mut self, value: u64) -> (ActionValue, ActionValue) {
        let old_max_log_entries = self.max_log_entries;
        self.max_log_entries = value;
        (
            ActionValue::Number(old_max_log_entries),
            ActionValue::Number(value),
        )
    }

    pub fn set_publicity(&mut self, value: bool) -> (ActionValue, ActionValue) {
        let old_publicity = self.is_public;
        self.is_public = value;
        (ActionValue::Bool(old_publicity), ActionValue::Bool(value))
    }

    pub fn set_owner(&mut self, value: Account) -> (ActionValue, ActionValue) {
        let old_owner = self.owner;
        self.owner = value;
        (
            ActionValue::Principal(old_owner.owner),
            ActionValue::Principal(value.owner),
        )
    }

    pub fn set_hotkeys_enabled(&mut self, value: bool) -> (ActionValue, ActionValue) {
        let old_hotkeys = self.hotkeys_enabled;
        self.hotkeys_enabled = value;
        (ActionValue::Bool(old_hotkeys), ActionValue::Bool(value))
    }
}
