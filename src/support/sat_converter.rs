use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use std::ops::{Add, Sub};
use std::ops::{Mul, Div};

#[allow(dead_code)]
const BTC_DECIMALS: u32 = 8;
pub const STH_DECIMALS: u32 = 8;
#[allow(dead_code)]
const USDT_DECIMALS: u32 = 8;

pub const SATS_PER_UNIT: i64 = 100_000_000;

/// # SatsConverter examples
///
/// ```rust,ignore
/// use rust_decimal::Decimal;
///
/// // from sats → BTC
/// let amount = SatsConverter::from_sats(100_000_000);
/// assert_eq!(amount.as_btc().to_string(), "1");
///
/// // from BTC → sats
/// let btc = Decimal::new(15, 1); // 1.5 BTC
/// let amount = SatsConverter::from_btc(btc).unwrap();
/// assert_eq!(amount.as_sats(), 150_000_000);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SatsConverter {
    sats: i64,
}

impl SatsConverter {
    /// Create from satoshis (i64)
    pub fn from_sats(sats: i64) -> Self {
        Self { sats }
    }

    /// Create from human-readable BTC (Decimal)
    pub fn from_btc(btc: Decimal) -> Option<Self> {
        let sats = (btc * Decimal::from_i64(SATS_PER_UNIT)?)
            .round()
            .to_i64()?;

        Some(Self { sats })
    }

    /// Get value in satoshis
    pub fn as_sats(&self) -> i64 {
        self.sats
    }

    /// Get human-readable BTC
    pub fn as_btc(&self) -> Decimal {
        Decimal::new(self.sats, STH_DECIMALS)
    }

    /// Validate if the satoshi amount is within a range (inclusive)
    pub fn validate_range(&self, min_sats: i64, max_sats: i64) -> bool {
        self.sats >= min_sats && self.sats <= max_sats
    }

    /// Alternative: returns Result for easier error handling
    pub fn ensure_in_range(&self, min_sats: i64, max_sats: i64) -> Result<(), String> {
        if self.sats < min_sats || self.sats > max_sats {
            Err(format!(
                "Amount {} sats is out of range ({} - {})",
                self.sats, min_sats, max_sats
            ))
        } else {
            Ok(())
        }
    }
}

impl Add for SatsConverter {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        self.sats.checked_add(rhs.sats).map(Self::from_sats)
    }
}

impl Sub for SatsConverter {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sats.checked_sub(rhs.sats).map(Self::from_sats)
    }
}

impl Mul<i64> for SatsConverter {
    type Output = Option<Self>;

    fn mul(self, rhs: i64) -> Self::Output {
        self.sats.checked_mul(rhs).map(Self::from_sats)
    }
}

impl Div<i64> for SatsConverter {
    type Output = Option<Self>;

    fn div(self, rhs: i64) -> Self::Output {
        if rhs == 0 {
            return None;
        }
        self.sats.checked_div(rhs).map(Self::from_sats)
    }
}

impl Mul<Decimal> for SatsConverter {
    type Output = Option<Self>;

    fn mul(self, rhs: Decimal) -> Self::Output {
        let btc = self.as_btc() * rhs;
        SatsConverter::from_btc(btc)
    }
}

impl Div<Decimal> for SatsConverter {
    type Output = Option<Self>;

    fn div(self, rhs: Decimal) -> Self::Output {
        if rhs.is_zero() {
            return None;
        }
        let btc = self.as_btc() / rhs;
        SatsConverter::from_btc(btc)
    }
}
