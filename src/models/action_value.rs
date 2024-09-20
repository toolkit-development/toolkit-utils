use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, Default)]
pub enum ActionValue {
    #[default]
    None,
    String(String),
    U64(u64),
    Principal(Principal),
    Bytes(Vec<u8>),
    Bool(bool),
    Time(u64),
    Unknown(String),
}
