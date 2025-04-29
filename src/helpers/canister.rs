use candid::Principal;
use ic_cdk::management_canister::{
    canister_status, create_canister_with_extra_cycles, install_code, CanisterInstallMode,
    CanisterSettings, CanisterStatusArgs, CreateCanisterArgs, InstallCodeArgs,
};

use crate::{api_error::ApiError, result::CanisterResult};

pub async fn deploy_canister(
    cycles: u64,
    controllers: Vec<Principal>,
) -> CanisterResult<Principal> {
    let args = CreateCanisterArgs {
        settings: Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            wasm_memory_limit: None,
            log_visibility: None,
            wasm_memory_threshold: None,
        }),
    };

    create_canister_with_extra_cycles(&args, cycles as u128)
        .await
        .map(|result| result.canister_id)
        .map_err(|err| {
            ApiError::external_service_error(&err.to_string())
                .add_method_name("deploy_canister")
                .add_source("toolkit_utils")
        })
}

pub async fn install_canister(
    canister_id: Principal,
    wasm_module: Vec<u8>,
    mode: CanisterInstallMode,
    arg: Option<Vec<u8>>,
) -> CanisterResult<Principal> {
    let install_args = InstallCodeArgs {
        mode,
        canister_id,
        wasm_module,
        arg: arg.unwrap_or_default(),
    };

    install_code(&install_args).await.map_err(|err| {
        ApiError::external_service_error(&err.to_string())
            .add_method_name("install_canister")
            .add_source("toolkit_utils")
    })?;

    Ok(canister_id)
}

pub async fn get_controllers(canister_id: Principal) -> Vec<Principal> {
    canister_status(&CanisterStatusArgs { canister_id })
        .await
        .map(|response| response.settings.controllers)
        .unwrap_or_default()
}
