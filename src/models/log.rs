use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::{Deserialize, Serialize};

use crate::{impl_storable_for, misc::generic::Time};

use super::action_value::ActionValue;

impl_storable_for!(Log);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Log {
    initial_value: ActionValue,
    action: String,
    action_value: ActionValue,
    initiated_by: Principal,
    created_at: Time,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            initial_value: Default::default(),
            action: Default::default(),
            action_value: Default::default(),
            initiated_by: caller(),
            created_at: time(),
        }
    }
}

impl Log {
    pub fn new(action: String, action_value: ActionValue) -> Self {
        Self {
            initial_value: ActionValue::None,
            action,
            action_value,
            initiated_by: caller(),
            created_at: time(),
        }
    }

    pub fn set_initial_value(&mut self, value: ActionValue) -> Self {
        self.initial_value = value;
        self.clone()
    }

    pub fn to_response(&self, id: u64) -> LogResponse {
        LogResponse {
            id,
            initial_value: self.initial_value.clone(),
            action: self.action.clone(),
            action_value: self.action_value.clone(),
            initiated_by: self.initiated_by,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct LogResponse {
    pub id: u64,
    pub initial_value: ActionValue,
    pub action: String,
    action_value: ActionValue,
    pub initiated_by: Principal,
    pub created_at: Time,
}
