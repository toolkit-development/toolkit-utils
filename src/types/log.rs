use std::collections::HashMap;

use candid::{CandidType, Principal};
use ic_cdk::api::{msg_caller, time};
use serde::{Deserialize, Serialize};

use crate::{impl_storable_for, misc::generic::Time};

use super::action_value::ActionValue;

impl_storable_for!(Log);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Log {
    action: String,
    changes: HashMap<String, Changevalues>,
    initiated_by: Principal,
    created_at: Time,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            changes: Default::default(),
            action: Default::default(),
            initiated_by: msg_caller(),
            created_at: time(),
        }
    }
}

impl Log {
    pub fn new(action: &str) -> Self {
        Self {
            changes: Default::default(),
            action: action.to_string(),
            initiated_by: msg_caller(),
            created_at: time(),
        }
    }

    pub fn add_change(&mut self, key: &str, changes: Changevalues) -> Self {
        self.changes.insert(key.to_string(), changes);
        self.clone()
    }

    pub fn to_response(&self, id: u64) -> LogResponse {
        let changes = self
            .changes
            .iter()
            .map(|(key, changes)| Change {
                action: key.clone(),
                initial: changes.initial.clone(),
                new: changes.new.clone(),
            })
            .collect();

        LogResponse {
            id,
            changes,
            initiated_by: self.initiated_by,
            created_at: self.created_at,
            action: self.action.clone(),
        }
    }
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Changevalues {
    pub initial: Option<ActionValue>,
    pub new: ActionValue,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Change {
    action: String,
    initial: Option<ActionValue>,
    new: ActionValue,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct LogResponse {
    pub id: u64,
    pub action: String,
    pub changes: Vec<Change>,
    pub initiated_by: Principal,
    pub created_at: Time,
}
