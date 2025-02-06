use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{governance_types::ProposalType, impl_storable_for};

use super::governance_types::GovernanceType;

impl_storable_for!(GovernanceConfig);

pub static DEFAULT_PROPOSAL_DURATION_SECONDS: u64 = 60 * 60 * 24 * 2; // 2 days in seconds
pub static PROPOSAL_DURATION_LOWER_LIMIT_SECONDS: u64 = 60 * 60 * 24; // 1 day in seconds
pub static PROPOSAL_DURATION_UPPER_LIMIT_SECONDS: u64 = 60 * 60 * 24 * 4; // 4 days in seconds

pub static CANISTER_STATUS_FETCH_INTERVAL_LOWER_LIMIT_SECONDS: u64 = 60 * 5; // 5 minutes in seconds
pub static CANISTER_STATUS_FETCH_INTERVAL_UPPER_LIMIT_SECONDS: u64 = 60 * 60 * 24 * 365; // 1 year in seconds

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct GovernanceConfig {
    pub proposal_duration_seconds: u64,
    pub governance_type: Option<String>,
    pub management_canister_id: Principal,
    pub deployer_canister_id: Principal,
    pub owner: Account,
    pub hotkeys_enabled: bool,
    pub upgraded_at: u64,
    pub deployed_at: u64,
}

impl GovernanceConfig {
    pub fn new(owner: Principal, management_canister_id: Principal) -> Self {
        Self {
            hotkeys_enabled: true,
            governance_type: None,
            deployer_canister_id: caller(),
            proposal_duration_seconds: DEFAULT_PROPOSAL_DURATION_SECONDS,
            management_canister_id,
            owner: Account::from(owner),
            upgraded_at: time(),
            deployed_at: time(),
        }
    }

    pub fn set_upgraded_at(&mut self, value: u64) -> () {
        self.upgraded_at = value;
    }

    pub fn set_governance(&mut self, value: GovernanceType) -> () {
        if let GovernanceType::None = value {
            self.governance_type = None
        } else {
            self.governance_type = Some(value.to_string());
        }
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

    pub fn set_proposal_duration_seconds(&mut self, value: u64) -> () {
        self.proposal_duration_seconds = value;
    }

    pub fn set_owner(&mut self, value: Account) -> () {
        self.owner = value;
    }

    pub fn set_hotkeys_enabled(&mut self, value: bool) -> () {
        self.hotkeys_enabled = value;
    }
}
