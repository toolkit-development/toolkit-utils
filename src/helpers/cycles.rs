use candid::Nat;
use ic_ledger_types::MAINNET_CYCLES_MINTING_CANISTER_ID;

use crate::{
    api_error::ApiError,
    cycles_minting::CyclesMintingService,
    misc::generic::{ICP_E8S, TRILLION_CYCLES},
    result::CanisterResult,
};

use super::misc::{e12s_to_f64, e8s_to_f64, f64_to_e8s, nat_to_u64};

pub async fn cycles_per_icp() -> CanisterResult<Nat> {
    let cycles_minting_service = CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID);
    let result = cycles_minting_service
        .get_icp_xdr_conversion_rate()
        .await
        .map(|(rate,)| rate)
        .map_err(|_| {
            ApiError::external_service_error("Error getting XDR conversion rate")
                .add_method_name("cycles_per_icp")
                .add_info("toolkit_utils")
        })?;

    Ok(Nat::from(
        (result.data.xdr_permyriad_per_icp * TRILLION_CYCLES) / 10_000,
    ))
}

pub async fn xdr_permyriad_per_icp() -> CanisterResult<u64> {
    let cycles_minting_service = CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID);
    let result = cycles_minting_service
        .get_icp_xdr_conversion_rate()
        .await
        .map(|(rate,)| rate)
        .map_err(|_| {
            ApiError::external_service_error("Error getting XDR conversion rate")
                .add_method_name("xdr_permyriad_per_icp")
                .add_info("toolkit_utils")
        })?;

    Ok(result.data.xdr_permyriad_per_icp)
}

pub async fn calculate_icp_fee_in_e8s(xdr_fee: u64) -> CanisterResult<u64> {
    // Step 1: Retrieve the XDR conversion rate (XDR per ICP).
    let cycles_minting_service = CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID);
    let result = cycles_minting_service
        .get_icp_xdr_conversion_rate()
        .await
        .map(|(rate,)| rate)
        .map_err(|_| {
            ApiError::external_service_error("Error getting XDR conversion rate")
                .add_method_name("calculate_icp_fee_in_e8s")
                .add_info("toolkit_utils")
        })?;

    // Step 2: Calculate ICP fee in e8s from XDR.
    let xdr_per_icp = result.data.xdr_permyriad_per_icp; // typically around 10,000 for 1 XDR.
    let icp_fee_e8s = (xdr_fee * ICP_E8S) / xdr_per_icp; // Converts fee to ICP in e8s

    Ok(icp_fee_e8s)
}

pub async fn cycles_per_icp_e8s(e8s: Nat) -> CanisterResult<u64> {
    let cycles_per_icp = e8s_to_f64(&cycles_per_icp().await?);
    let tokens_e8s = e8s_to_f64(&e8s);

    let cycles = (tokens_e8s) * cycles_per_icp;
    Ok(nat_to_u64(&f64_to_e8s(cycles)))
}

pub async fn icp_per_cycles_e12s(e12s: Nat) -> CanisterResult<Nat> {
    let cycles_per_icp = e8s_to_f64(&cycles_per_icp().await?);
    let tokens_e12s = e12s_to_f64(&e12s);

    let icp = tokens_e12s / (cycles_per_icp / 10_000f64);
    Ok(f64_to_e8s(icp))
}
