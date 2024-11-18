use candid::Principal;
use ic_cdk::{caller, id};

use crate::{api_error::ApiError, canister::get_controllers};

/// Validates if the caller is a controller of the canister.
///
/// This function retrieves the list of controllers for the canister
/// and checks if the caller is included in the list.
///
/// # Returns
///
/// A `Result<(), String>` indicating success if the caller is a controller.
///
/// # Errors
///
/// - Returns a string error message if the caller is not a controller.
pub async fn is_controller() -> Result<(), String> {
    let controllers = get_controllers(id()).await;
    if controllers.contains(&caller()) {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Caller is not a controller")
        .to_string())
}

/// Ensures that the caller is not anonymous.
///
/// This function checks if the caller's principal is the anonymous principal.
///
/// # Returns
///
/// A `Result<(), String>` indicating success if the caller is not anonymous.
///
/// # Errors
///
/// - Returns a string error message if the caller is anonymous.
pub fn is_not_anonymous() -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err(ApiError::unauthorized()
            .add_message("Caller is anonymous")
            .to_string());
    }

    Ok(())
}

/// Ensures that the caller is an administrator.
///
/// This function checks if the caller's principal matches a predefined
/// list of administrator principals.
///
/// # Returns
///
/// A `Result<(), String>` indicating success if the caller is an admin.
///
/// # Errors
///
/// - Returns a string error message if the caller is not an admin.
pub fn is_admin() -> Result<(), String> {
    if ["vafd2-aurwj-5igu3-htth5-olb42-6ficf-ttehy-2oyrp-u6nsy-qjlay-7ae"]
        .contains(&caller().to_string().as_str())
    {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Caller is not an admin")
            .to_string())
    }
}
