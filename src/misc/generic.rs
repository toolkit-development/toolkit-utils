pub type Ballots = u64;
pub type Time = u64;

// cycles related
pub static TRILLION_CYCLES: u64 = 1_000_000_000_000; // 1 trillion cycles
pub static CANISTER_SPINUP_CYCLES: u64 = 500_000_000_000; // 0.5 trillion cycles

pub static CYCLES_SAFETY_MARGIN: u64 = 1_000_000_000; // 1 billion cycles

pub static MIN_CYCLES_FOR_PROJECT_CANISTER_SPINUP: u64 =
    (TRILLION_CYCLES * 10) + CANISTER_SPINUP_CYCLES;
pub static MIN_CYCLES_FOR_PROJECT_SPINUP: u64 = (MIN_CYCLES_FOR_PROJECT_CANISTER_SPINUP) * 2; // 2 canisters

pub static MIN_CYCLES_FOR_CANISTER_SPINUP: u64 = TRILLION_CYCLES * 3;

pub static XDR_FEE_FOR_CANISTER: u64 = 5_000; // 0.5 XDR
pub static XDR_FEE_FOR_PROJECT: u64 = 100_000; // 10 XDR

pub static MIN_CYCLES_FOR_CANISTER_SPINUP_DEV: u64 = (TRILLION_CYCLES) - CYCLES_SAFETY_MARGIN;
pub static XDR_FEE_FOR_CANISTER_DEV: u64 = 500; // 0.05 XDR
pub static XDR_FEE_FOR_CORE_DEV: u64 = 500; // 0.05 XDR

// cycles management canister related
pub static MEMO_TOP_UP_CANISTER: u64 = 0x50555054;
pub static MEMO_CREATE_CANISTER: u64 = 1095062083;

// ICP related
pub static ICP_TRANSACTION_FEE: u64 = 10_000;
pub static ICP_E8S: u64 = 100_000_000;

// MISC
pub static DAY_IN_SECONDS: u64 = 60 * 60 * 24;
pub static FEE_WALLET: &str = "j75hh-yqaaa-aaaap-akolq-cai";
