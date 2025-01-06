use candid::Principal;
use ic_cdk::{api, caller};

use crate::{api_error::ApiError, CanisterResult};

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
pub fn is_controller() -> CanisterResult<()> {
    if !api::is_controller(&caller()) {
        return Err(ApiError::forbidden("Caller is not a controller")
            .add_method_name("is_controller")
            .add_source("toolkit_utils"));
    }
    Ok(())
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
        return Err(ApiError::forbidden("Caller is anonymous")
            .add_method_name("is_controller")
            .add_source("toolkit_utils")
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
    if [
        "vafd2-aurwj-5igu3-htth5-olb42-6ficf-ttehy-2oyrp-u6nsy-qjlay-7ae",
        "tg7ak-dyvdw-wels6-b4hx3-vaooh-mxn7w-vqvvs-k4mab-ju56d-pqrbf-5qe",
        "jx573-d63v2-vmp75-c5lgs-evd2l-j2uft-hgxs6-6g7hx-hy4al-o4g3k-qae",
    ]
    .contains(&caller().to_string().as_str())
    {
        Ok(())
    } else {
        Err(ApiError::forbidden("Caller is not an admin")
            .add_method_name("is_admin")
            .add_source("toolkit_utils")
            .to_string())
    }
}
