use alloy_primitives::U256;
use anyhow::anyhow;
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::FromStr;

pub fn u256_to_decimal_raw(value: U256) -> anyhow::Result<Decimal> {
    Decimal::from_str(&value.to_string()).map_err(|e| anyhow!("U256 → Decimal conversion failed: {e}"))
}

pub fn u256_to_decimal_human(value: U256, decimals: u32) -> anyhow::Result<Decimal> {
    Ok(u256_to_decimal_raw(value)? / Decimal::from(10u64).powu(decimals as u64))
}

// /// This only works if the value fits into i128 (most balances do).
// fn u256_as_i128_to_decimal(value: U256, decimals: u32) -> Decimal {
//     Decimal::from_i128_with_scale(value.try_into().expect("U256 too large"), decimals)
// }

// /// No value.to_string() allocates
// fn u256_to_decimal_fast(value: U256, decimals: u32) -> Decimal {
//     let v: i128 = value.try_into().expect("U256 too large");
//     Decimal::from_i128_with_scale(v, decimals)
// }
