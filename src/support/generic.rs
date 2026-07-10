use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

const MAX_PRECISION: u32 = 20;

/// # Examples
///
/// ```rust,ignore
/// use rust_decimal::Decimal;
///
/// // from string
/// let amount = MoneyAmount::from_str("666.20408163").unwrap();
/// assert_eq!(amount.to_string_fixed(), "666.20408163");
///
/// // from Decimal
/// let dec = Decimal::new(123456789, 8); // 1.23456789
/// let amount = MoneyAmount::from_decimal(dec).unwrap();
///
/// // DB-safe (NUMERIC(20,8))
/// let db_value: Decimal = amount.as_decimal();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoneyAmount<const SCALE: u32> {
    value: Decimal,
}

impl<const SCALE: u32> PartialOrd for MoneyAmount<SCALE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<const SCALE: u32> MoneyAmount<SCALE> {
    /// Create from Decimal, enforcing scale and precision
    pub fn from_decimal(mut value: Decimal) -> Option<Self> {
        // Normalize scale (round half-up to 8 decimals)
        value = value.round_dp(SCALE);

        // Enforce scale exactly
        if value.scale() > SCALE {
            return None;
        }

        // Enforce NUMERIC(20,8): max integer digits = 12
        let integer_digits = value
            .trunc()
            .to_string()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .count();

        if integer_digits > (MAX_PRECISION - SCALE) as usize {
            return None;
        }

        Some(Self { value })
    }

    /// Create from string like "666.20408163"
    pub fn from_str(s: &str) -> Option<Self> {
        let dec = Decimal::from_str(s).ok()?;
        Self::from_decimal(dec)
    }

    /// Get underlying Decimal (for DB / math)
    pub fn as_decimal(&self) -> Decimal {
        self.value
    }

    /// String representation (always 8 decimals)
    pub fn to_string_fixed(&self) -> String {
        format!("{:.*}", SCALE as usize, self.value)
    }

    /// Returns the underlying integer representation (scaled by 10^SCALE).
    pub fn to_raw(self) -> Option<i128> {
        let factor = Decimal::from(10u64.pow(SCALE));
        self.value.checked_mul(factor)?.trunc().to_i128()
    }

    /// Creates a MoneyAmount from a raw integer scaled by 10^8.
    pub fn from_raw(raw: i128) -> Option<Self> {
        Self::from_decimal(Decimal::from_i128_with_scale(raw, SCALE))
    }

    /// Validate range (inclusive)
    pub fn validate_range(&self, min: Decimal, max: Decimal) -> bool {
        self.value >= min && self.value <= max
    }

    /// Result-based validation
    pub fn ensure_in_range(
        &self,
        min: Decimal,
        max: Decimal,
    ) -> Result<(), String> {
        if self.value < min || self.value > max {
            Err(format!(
                "Amount {} is out of range ({} - {})",
                self.value, min, max
            ))
        } else {
            Ok(())
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// Check if the amount is strictly positive (> 0)
    pub fn is_positive(&self) -> bool {
        self.value > Decimal::ZERO
    }
}

impl<const SCALE: u32> Add for MoneyAmount<SCALE> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        MoneyAmount::from_decimal(self.value + rhs.value)
    }
}

impl<const SCALE: u32> Sub for MoneyAmount<SCALE> {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        MoneyAmount::from_decimal(self.value - rhs.value)
    }
}

impl<const SCALE: u32> MoneyAmount<SCALE> {
    pub fn checked_add(self, rhs: Self) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value + rhs.value)
            .ok_or("overflow or precision error")
    }

    pub fn checked_mul(self, factor: Decimal) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value * factor)
            .ok_or("overflow or precision error")
    }
}
