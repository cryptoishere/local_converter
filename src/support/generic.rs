use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::str::FromStr;
use rust_decimal::MathematicalOps;

const MAX_PRECISION: u32 = 20;

/// A fixed‑scale monetary amount backed by `rust_decimal::Decimal`.
///
/// The scale (number of decimal places) is a compile‑time constant, ensuring
/// type‑safe handling of amounts with different precisions. Comparisons (`==`,
/// `>`, etc.) work both within the same scale and across the common scales
/// (0, 2, 4, 8) via generated operator overloads.
///
/// # Examples
/// ```rust,ignore
/// # use rust_decimal::Decimal;
/// # use money_amount::MoneyAmount;
/// // from string
/// let amount = MoneyAmount::<8>::from_str("666.20408163").unwrap();
/// assert_eq!(amount.to_string_fixed(), "666.20408163");
///
/// // from Decimal
/// let dec = Decimal::new(123456789, 8); // 1.23456789
/// let amount = MoneyAmount::<8>::from_decimal(dec).unwrap();
///
/// // DB-safe (NUMERIC(20,8))
/// let db_value: Decimal = amount.as_decimal();
///
/// // cross‑scale comparison works for supported scales
/// let a = MoneyAmount::<8>::from_str("1.23").unwrap();
/// let b = MoneyAmount::<4>::from_str("1.2300").unwrap();
/// assert!(a == b); // true
/// assert!(a <= b);
/// ```
#[derive(Debug, Clone, Copy, Eq)]
pub struct MoneyAmount<const SCALE: u32> {
    value: Decimal,
}

// Same‑scale comparisons
impl<const SCALE: u32> PartialEq for MoneyAmount<SCALE> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const SCALE: u32> PartialOrd for MoneyAmount<SCALE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<const SCALE: u32> Ord for MoneyAmount<SCALE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

// Cross‑scale comparisons (supported scale pairs)

/// Generates `PartialEq` and `PartialOrd` implementations for two different scales.
macro_rules! impl_cross_scale_ops {
    ($scale1:expr, $scale2:expr) => {
        impl PartialEq<MoneyAmount<$scale2>> for MoneyAmount<$scale1> {
            fn eq(&self, other: &MoneyAmount<$scale2>) -> bool {
                self.value == other.value
            }
        }
        impl PartialEq<MoneyAmount<$scale1>> for MoneyAmount<$scale2> {
            fn eq(&self, other: &MoneyAmount<$scale1>) -> bool {
                self.value == other.value
            }
        }
        impl PartialOrd<MoneyAmount<$scale2>> for MoneyAmount<$scale1> {
            fn partial_cmp(&self, other: &MoneyAmount<$scale2>) -> Option<Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }
        impl PartialOrd<MoneyAmount<$scale1>> for MoneyAmount<$scale2> {
            fn partial_cmp(&self, other: &MoneyAmount<$scale1>) -> Option<Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }
    };
}

impl_cross_scale_ops!(8, 6);
impl_cross_scale_ops!(8, 2);
impl_cross_scale_ops!(8, 0);
impl_cross_scale_ops!(6, 2);
impl_cross_scale_ops!(6, 0);

// Core constructors and accessors

impl<const SCALE: u32> MoneyAmount<SCALE> {
    /// Creates a `MoneyAmount` from a `Decimal`, enforcing the fixed scale and
    /// the total precision constraint (`MAX_PRECISION`).
    ///
    /// The value is rounded to the target scale using half‑up rounding.
    pub fn from_decimal(mut value: Decimal) -> Option<Self> {
        value = value.round_dp(SCALE);
        if value.scale() > SCALE {
            return None;
        }

        // Enforce total digits: integer part must fit in (MAX_PRECISION - SCALE)
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

    /// Parses a decimal string, e.g. `"666.20408163"`.
    pub fn from_str(s: &str) -> Option<Self> {
        let dec = Decimal::from_str(s).ok()?;
        Self::from_decimal(dec)
    }

    /// Returns the underlying `Decimal` value (useful for DB storage or further math).
    pub fn as_decimal(&self) -> Decimal {
        self.value
    }

    /// Formats the amount with exactly `SCALE` decimal places.
    pub fn to_string_fixed(&self) -> String {
        format!("{:.*}", SCALE as usize, self.value)
    }

    /// Converts the amount to a raw integer scaled by `10^SCALE`.
    ///
    /// # Errors
    /// Returns `None` if the value does not fit in `i128` or if `10^SCALE` overflows.
    pub fn to_raw(self) -> Option<i128> {
        // Use Decimal::TEN.powi() to avoid u64 overflow for large scales
        let factor = Decimal::TEN.powi(SCALE as i64);
        self.value.checked_mul(factor)?.trunc().to_i128()
    }

    /// Creates a `MoneyAmount` from a raw integer already scaled by `10^SCALE`.
    pub fn from_raw(raw: i128) -> Option<Self> {
        Self::from_decimal(Decimal::from_i128_with_scale(raw, SCALE))
    }

    /// Checks whether the amount is within the inclusive range `[min, max]`.
    pub fn validate_range(&self, min: Decimal, max: Decimal) -> bool {
        self.value >= min && self.value <= max
    }

    /// Returns `Ok(())` if the amount is within the inclusive range, else an error.
    pub fn ensure_in_range(&self, min: Decimal, max: Decimal) -> Result<(), String> {
        if self.value < min || self.value > max {
            Err(format!(
                "Amount {} is out of range ({} - {})",
                self.value, min, max
            ))
        } else {
            Ok(())
        }
    }

    /// Returns `true` if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// Returns `true` if the amount is strictly positive.
    pub fn is_positive(&self) -> bool {
        self.value > Decimal::ZERO
    }
}

// Arithmetic (same scale)

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
    /// Addition that returns a `Result` instead of an `Option`.
    pub fn checked_add(self, rhs: Self) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value + rhs.value)
            .ok_or("overflow or precision error")
    }

    /// Multiplication by a `Decimal` factor.
    pub fn checked_mul(self, factor: Decimal) -> Result<Self, &'static str> {
        MoneyAmount::from_decimal(self.value * factor)
            .ok_or("overflow or precision error")
    }
}

// Convenience traits

impl<const SCALE: u32> Default for MoneyAmount<SCALE> {
    fn default() -> Self {
        Self { value: Decimal::ZERO }
    }
}

impl<const SCALE: u32> From<MoneyAmount<SCALE>> for Decimal {
    fn from(amount: MoneyAmount<SCALE>) -> Self {
        amount.value
    }
}

impl<const SCALE: u32> TryFrom<Decimal> for MoneyAmount<SCALE> {
    type Error = &'static str;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        Self::from_decimal(value).ok_or("invalid scale or precision")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_scale_comparisons() {
        let a = MoneyAmount::<8>::from_str("1.23").unwrap();
        let b = MoneyAmount::<8>::from_str("2.34").unwrap();
        let c = MoneyAmount::<8>::from_str("1.23").unwrap();

        assert!(a < b);
        assert!(b > a);
        assert!(a == c);
        assert!(a != b);
        assert!(a <= c);
        assert!(b >= a);
    }

    #[test]
    fn cross_scale_comparisons() {
        let a = MoneyAmount::<8>::from_str("1.23").unwrap();
        let b = MoneyAmount::<2>::from_str("2.34").unwrap();
        let c = MoneyAmount::<8>::from_str("1.23").unwrap();

        assert!(a < b);
        assert!(b > a);
        assert!(a == c); // same value
        assert!(a != b);
    }

    #[test]
    fn same_value_different_scales() {
        let a = MoneyAmount::<8>::from_str("1.23000000").unwrap();
        let b = MoneyAmount::<2>::from_str("1.23").unwrap();
        assert!(a == b);
    }

    #[test]
    fn ordering_and_sort() {
        let mut amounts = vec![
            MoneyAmount::<8>::from_str("1.00").unwrap(),
            MoneyAmount::<8>::from_str("3.00").unwrap(),
            MoneyAmount::<8>::from_str("2.00").unwrap(),
        ];
        amounts.sort();
        assert_eq!(amounts[0].to_string_fixed(), "1.00000000");
        assert_eq!(amounts[1].to_string_fixed(), "2.00000000");
        assert_eq!(amounts[2].to_string_fixed(), "3.00000000");
    }

    #[test]
    fn arithmetic() {
        let a = MoneyAmount::<8>::from_str("1.23").unwrap();
        let b = MoneyAmount::<8>::from_str("2.34").unwrap();
        let sum = (a + b).unwrap();
        assert_eq!(sum.to_string_fixed(), "3.57000000");

        let diff = (b - a).unwrap();
        assert_eq!(diff.to_string_fixed(), "1.11000000");
    }

    #[test]
    fn raw_conversion() {
        let amount = MoneyAmount::<8>::from_str("123.45678901").unwrap();
        let raw = amount.to_raw().unwrap();
        let restored = MoneyAmount::<8>::from_raw(raw).unwrap();
        assert_eq!(amount, restored);
    }

    #[test]
    fn default_is_zero() {
        let zero = MoneyAmount::<8>::default();
        assert!(zero.is_zero());
        assert_eq!(zero.to_string_fixed(), "0.00000000");
    }

    #[test]
    fn try_from_decimal() {
        let dec = Decimal::new(123456789, 8);
        let amount = MoneyAmount::<8>::try_from(dec).unwrap();
        assert_eq!(amount.as_decimal(), dec);
    }
}
