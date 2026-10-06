use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Money(i64);
impl Money {
    pub const ZERO: Self = Self(0);

    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    pub fn from_yuan(yuan: i64) -> Option<Self>{
        yuan.checked_mul(100).map(Self)
    }

    pub fn cents(self) -> i64 {
        self.0
    }

    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub fn checked_add(self, rhs: Self) -> Option<Money> {
        self.0.checked_add(rhs.0).map(Self)
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Money> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs = self.0.unsigned_abs();
        if self.0 < 0 {
            write!(f, "-{}.{:02}", abs / 100, abs % 100)
        }else {
            write!(f, "{}.{:02}", abs / 100, abs % 100)
        }
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a+ b)
    }
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_from_yuan_overflow() {
        assert_eq!(Money::from_yuan(i64::MAX), None);
        assert_eq!(Money::from_yuan(100), Some(Money::from_cents(10000)));
    }

    #[test]
    fn test_checked_add_sub_overflow() {
        let max = Money::from_cents(i64::MAX);
        assert_eq!(max.checked_add(Money::from_cents(1)), None);

        let min = Money::from_cents(i64::MIN);
        assert_eq!(min.checked_sub(Money::from_cents(1)), None);
    }

    #[test]
    fn test_display_min_does_not_panic() {
        let min = Money::from_cents(i64::MIN);
        let s = format!("{min}");
        assert_eq!(s, "-92233720368547758.08");
    }
}