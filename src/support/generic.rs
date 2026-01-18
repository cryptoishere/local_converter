use rust_decimal::Decimal;
use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

const MONEY_SCALE: u32 = 8;
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
pub struct MoneyAmount {
    value: Decimal, // always scale = 8
}

impl PartialOrd for MoneyAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl MoneyAmount {
    /// Create from Decimal, enforcing scale and precision
    pub fn from_decimal(mut value: Decimal) -> Option<Self> {
        // Normalize scale (round half-up to 8 decimals)
        value = value.round_dp(MONEY_SCALE);

        // Enforce scale exactly
        if value.scale() > MONEY_SCALE {
            return None;
        }

        // Enforce NUMERIC(20,8): max integer digits = 12
        let integer_digits = value
            .trunc()
            .to_string()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .count();

        if integer_digits > (MAX_PRECISION - MONEY_SCALE) as usize {
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
        format!("{:.*}", MONEY_SCALE as usize, self.value)
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
}

impl Add for MoneyAmount {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        MoneyAmount::from_decimal(self.value + rhs.value)
    }
}

impl Sub for MoneyAmount {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        MoneyAmount::from_decimal(self.value - rhs.value)
    }
}

impl MoneyAmount {
    pub fn checked_add(self, rhs: Self) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value + rhs.value)
            .ok_or("overflow or precision error")
    }

    pub fn checked_mul(self, factor: Decimal) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value * factor)
            .ok_or("overflow or precision error")
    }
}
