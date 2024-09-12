use candid::{Nat, Principal};
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT};

pub fn principal_to_account_identifier(principal: Principal) -> AccountIdentifier {
    AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT)
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

pub fn e12s_to_f64(n: &Nat) -> f64 {
    nat_to_f64(n) / 1000000000000.0
}

pub fn format_with_underscores(value: u64) -> String {
    let s = value.to_string();
    let mut result = String::new();

    for (count, c) in s.chars().rev().enumerate() {
        if count != 0 && count % 3 == 0 {
            result.push('_');
        }
        result.push(c);
    }

    result.chars().rev().collect()
}
