use ic_ledger_types::{Memo, Tokens};

pub type Ballots = u64;
pub type Time = u64;

pub static TRILLION_CYCLES: u64 = 1_000_000_000_000;
pub static MEMO_TOP_UP_CANISTER: Memo = Memo(1347768404_u64);
pub static MEMO_CREATE_CANISTER: Memo = Memo(1095062083_u64);
pub static ICP_TRANSACTION_FEE: Tokens = Tokens::from_e8s(10000);
