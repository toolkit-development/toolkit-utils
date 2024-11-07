use ic_ledger_types::{Memo, Tokens};

pub type Ballots = u64;
pub type Time = u64;

// cycles related
pub static TRILLION_CYCLES: u64 = 1_000_000_000_000;
pub static SAFETY_MARGIN: u64 = 1_000_000_000;
pub static MIN_CYCLES_FOR_CANISTER_SPINUP: u64 = (TRILLION_CYCLES * 3) - SAFETY_MARGIN;

// cycles management canister related
pub static MEMO_TOP_UP_CANISTER: Memo = Memo(1347768404);
pub static MEMO_CREATE_CANISTER: Memo = Memo(1095062083);

// ICP related
pub static ICP_TRANSACTION_FEE: Tokens = Tokens::from_e8s(10000);

// MISC
pub static DAY_IN_SECONDS: u64 = 60 * 60 * 24;
pub static FEE_WALLET: &str = "fcygz-gqaaa-aaaap-abpaa-cai";
