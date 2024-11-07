use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, Default)]
pub enum ActionValue {
    #[default]
    None,
    String(String),
    Number(u64),
    Principal(Principal),
    Account(Account),
    Bytes(Vec<u8>),
    Bool(bool),
    Time(u64),
    Unknown(String),
}
