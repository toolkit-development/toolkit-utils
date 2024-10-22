use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::CanisterResult;

use super::api_error::ApiError;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct CustomRootInitArgs {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub website: Option<String>,
}

impl CustomRootInitArgs {
    pub fn validate(&self) -> CanisterResult<()> {
        if self.name.len() < 3 {
            return Err(ApiError::bad_request().add_message("Name must be at least 3 characters"));
        }

        if self.logo.is_empty() {
            return Err(ApiError::bad_request().add_message("Logo is not a valid base64 string."));
        }

        Ok(())
    }
}
