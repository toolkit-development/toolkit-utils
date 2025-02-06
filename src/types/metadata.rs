use candid::{CandidType, Principal};
use ic_cdk::api::time;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{impl_storable_for, misc::generic::Time};

use super::{
    action_value::ActionValue, project_root_init_args::ProjectInitArgs, result::CanisterResult,
};

impl_storable_for!(Metadata);

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub url: Option<String>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_by: Account,
    pub created_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateMetadata {
    pub url: Option<String>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct MetadataResponse {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub url: Option<String>,
    pub created_by: Account,
    pub created_at: Time,
    pub updated_at: Time,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            logo: Default::default(),
            url: None,

            created_by: Account::from(Principal::anonymous()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl Metadata {
    pub fn new(
        name: String,
        description: String,
        logo: String,
        url: Option<String>,
        created_by: Principal,
    ) -> Self {
        Self {
            name: Some(name),
            description: Some(description),
            logo: Some(logo),

            created_by: Account::from(created_by),
            url,
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn update_metadata(&mut self, metadata: UpdateMetadata) -> CanisterResult<()> {
        let data = ProjectInitArgs {
            name: metadata
                .name
                .unwrap_or(self.name.clone().unwrap_or_default()),
            description: metadata
                .description
                .unwrap_or(self.description.clone().unwrap_or_default()),
            logo: metadata
                .logo
                .unwrap_or(self.logo.clone().unwrap_or_default()),
            url: Some(metadata.url.unwrap_or(self.url.clone().unwrap_or_default())),
        };

        data.validate()?;

        self.name = Some(data.name);
        self.description = Some(data.description);
        self.logo = Some(data.logo);
        self.url = data.url;
        self.updated_at = time();
        Ok(())
    }

    pub fn update_name(&mut self, name: String) -> (ActionValue, ActionValue) {
        let old_name = self.name.clone().unwrap_or_default();
        self.name = Some(name.clone());
        self.updated_at = time();
        (ActionValue::String(old_name), ActionValue::String(name))
    }

    pub fn update_description(&mut self, description: String) -> (ActionValue, ActionValue) {
        let old_description = self.description.clone().unwrap_or_default();
        self.description = Some(description.clone());
        self.updated_at = time();
        (
            ActionValue::String(old_description),
            ActionValue::String(description),
        )
    }

    pub fn update_logo(&mut self, logo: String) -> (ActionValue, ActionValue) {
        let old_logo = self.logo.clone().unwrap_or_default();
        self.logo = Some(logo.clone());
        self.updated_at = time();
        (ActionValue::String(old_logo), ActionValue::String(logo))
    }

    pub fn update_website(&mut self, website: Option<String>) -> (ActionValue, ActionValue) {
        let old_website = self.url.clone();
        self.url = website.clone();
        self.updated_at = time();
        (
            ActionValue::String(old_website.unwrap_or_default()),
            ActionValue::String(website.unwrap_or_default()),
        )
    }

    pub fn to_response(&self) -> MetadataResponse {
        MetadataResponse {
            name: self.name.clone().unwrap_or_default(),
            description: self.description.clone().unwrap_or_default(),
            logo: self.logo.clone().unwrap_or_default(),
            url: self.url.clone(),
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
