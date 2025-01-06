use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::CanisterResult;

use super::api_error::ApiError;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ProjectInitArgs {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub website: Option<String>,
}

impl ProjectInitArgs {
    pub fn validate(&self) -> CanisterResult<()> {
        if self.name.len() < 3 {
            return Err(ApiError::bad_request("Name must be at least 3 characters")
                .add_method_name("validate")
                .add_source("toolkit_utils"));
        }

        if self.logo.is_empty() {
            return Err(ApiError::bad_request("Logo is not a valid base64 string.")
                .add_method_name("validate")
                .add_source("toolkit_utils"));
        }

        Ok(())
    }
}
