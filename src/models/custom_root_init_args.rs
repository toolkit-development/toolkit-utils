use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct CustomRootInitArgs {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub website: Option<String>,
}
