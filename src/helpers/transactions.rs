use candid::Principal;
use ic_cdk::api::{call::CallResult, time};
use ic_ledger_types::{
    account_balance, query_archived_blocks, query_blocks, transfer, AccountBalanceArgs,
    AccountIdentifier, Block, BlockIndex, GetBlocksArgs, Memo, Operation, Timestamp, Tokens,
    TransferArgs, TransferResult, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use crate::{api_error::ApiError, CanisterResult};

pub async fn transfer_icp(to: Principal, amount_e8s: u64) -> CallResult<TransferResult> {
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

    transfer(MAINNET_LEDGER_CANISTER_ID, args).await
}

pub async fn get_icp_balance(principal: Principal) -> CallResult<Tokens> {
    let args = AccountBalanceArgs {
        account: principal_to_account_identifier(principal),
    };

    account_balance(MAINNET_LEDGER_CANISTER_ID, args).await
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

fn principal_to_account_identifier(principal: Principal) -> AccountIdentifier {
    AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT)
}
