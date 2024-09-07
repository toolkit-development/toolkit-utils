use candid::{Nat, Principal};
use ic_cdk::{api::time, id};
use ic_ledger_types::{
    account_balance, query_archived_blocks, query_blocks, transfer, AccountBalanceArgs,
    AccountIdentifier, Block, BlockIndex, GetBlocksArgs, Memo, Operation, Subaccount, Timestamp,
    Tokens, TransferArgs, DEFAULT_SUBACCOUNT, MAINNET_CYCLES_MINTING_CANISTER_ID,
    MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    api_error::ApiError,
    cycles_minting::{CyclesMintingService, NotifyTopUpArg, NotifyTopUpResult},
    misc::generic::{ICP_TRANSACTION_FEE, MEMO_TOP_UP_CANISTER, TRILLION_CYCLES},
    CanisterResult,
};

pub async fn transfer_icp(to: Principal, amount_e8s: u64) -> CanisterResult<BlockIndex> {
    let args = TransferArgs {
        to: principal_to_account_identifier(to),
        amount: Tokens::from_e8s(amount_e8s),
        fee: Tokens::from_e8s(10_000),
        memo: Memo(0),
        from_subaccount: None,
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn self_top_up_cycles(icp_amount: u64) -> CanisterResult<Nat> {
    let block_index = top_up_cycles(icp_amount, id()).await?;
    notify_top_up_cycles(block_index).await
}

pub async fn top_up_cycles(icp_amount: u64, canister: Principal) -> CanisterResult<BlockIndex> {
    let amount = Tokens::from_e8s(icp_amount) - ICP_TRANSACTION_FEE;

    let args = TransferArgs {
        memo: MEMO_TOP_UP_CANISTER,
        amount,
        fee: ICP_TRANSACTION_FEE,
        from_subaccount: None,
        to: AccountIdentifier::new(
            &MAINNET_CYCLES_MINTING_CANISTER_ID,
            &Subaccount::from(canister),
        ),
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    match transfer(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(result) => match result {
            Ok(block_index) => Ok(block_index),
            Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
        },
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn notify_top_up_cycles(block_index: u64) -> CanisterResult<Nat> {
    match CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID)
        .notify_top_up(NotifyTopUpArg {
            block_index,
            canister_id: id(),
        })
        .await
    {
        Ok((result,)) => match result {
            NotifyTopUpResult::Ok(cycles) => Ok(cycles),
            NotifyTopUpResult::Err(_) => {
                Err(ApiError::bad_request().add_message("Error notifying top up"))
            }
        },
        Err((_, err)) => Err(ApiError::bad_request().add_message(&err)),
    }
}

pub async fn get_icp_balance(principal: Principal) -> CanisterResult<Tokens> {
    let args = AccountBalanceArgs {
        account: principal_to_account_identifier(principal),
    };

    match account_balance(MAINNET_LEDGER_CANISTER_ID, args).await {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(ApiError::unexpected().add_message(format!("{:?}", err))),
    }
}

pub async fn validate_transaction(
    from_principal: Principal,
    to_principal: Principal,
    block_index: BlockIndex,
) -> Option<Tokens> {
    let block = get_block(block_index).await?;

    match block.transaction.operation? {
        Operation::Transfer {
            from,
            to,
            amount,
            fee: _, // Ignore fee
        } => {
            if from != principal_to_account_identifier(from_principal) {
                return None;
            }
            if to != principal_to_account_identifier(to_principal) {
                return None;
            }
            Some(amount)
        }
        _ => None,
    }
}

async fn get_block(block_index: BlockIndex) -> Option<Block> {
    let args = GetBlocksArgs {
        start: block_index,
        length: 1,
    };

    if let Ok(blocks_result) = query_blocks(MAINNET_LEDGER_CANISTER_ID, args.clone()).await {
        if !blocks_result.blocks.is_empty() {
            return blocks_result.blocks.into_iter().next();
        }

        if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
            (b.start <= block_index && (block_index - b.start) < b.length).then_some(b.callback)
        }) {
            if let Ok(range) = query_archived_blocks(&func, args).await {
                match range {
                    Ok(_range) => return _range.blocks.into_iter().next(),
                    Err(_) => return None,
                }
            }
        }
    }

    None
}

pub async fn get_cycles_per_icp() -> CanisterResult<Nat> {
    let cycles_minting_service = CyclesMintingService(MAINNET_CYCLES_MINTING_CANISTER_ID);
    let result = cycles_minting_service
        .get_icp_xdr_conversion_rate()
        .await
        .map(|(rate,)| rate)
        .map_err(|_| ApiError::bad_request().add_message("Error getting XDR conversion rate"))?;

    Ok(Nat::from(
        (result.data.xdr_permyriad_per_icp * TRILLION_CYCLES) / 10_000,
    ))
}

fn principal_to_account_identifier(principal: Principal) -> AccountIdentifier {
    AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT)
}

pub async fn calculate_total_cycles_available(icp_amount: Nat) -> CanisterResult<Nat> {
    let cycles_per_icp = get_cycles_per_icp().await?;
    let calc = e8s_to_f64(&icp_amount) * nat_to_f64(&cycles_per_icp);
    Ok(Nat::from(calc.round() as u64))
}

pub async fn calculate_total_icp_required(cycles: Nat) -> CanisterResult<Nat> {
    let cycles_per_icp = get_cycles_per_icp().await?;
    let calc = e8s_to_f64(&cycles) / nat_to_f64(&cycles_per_icp);
    Ok(f64_to_e8s(calc))
}

pub fn nat_to_f64(n: &Nat) -> f64 {
    let n_str = n.0.to_string();
    n_str.parse::<f64>().unwrap()
}

pub fn f64_to_u64(f: f64) -> u64 {
    f.round() as u64
}

pub fn nat_to_u64(n: &Nat) -> u64 {
    f64_to_u64(nat_to_f64(n))
}

pub fn f64_to_e8s(f: f64) -> Nat {
    Nat::from((f * 1e8) as u128)
}

pub fn e8s_to_f64(n: &Nat) -> f64 {
    nat_to_f64(n) / 100000000.0
}
