use candid::Principal;
use ic_cdk::{caller, id};

use crate::{api_error::ApiError, canister::get_controllers};

pub async fn is_controller() -> Result<(), String> {
    let controllers = get_controllers(id()).await;
    if controllers.contains(&caller()) {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Caller is not a controller")
        .to_string())
}

pub fn is_not_anonymous() -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err(ApiError::unauthorized()
            .add_message("Caller is anonymous")
            .to_string());
    }

    Ok(())
}
