use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Number(pub f64);

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number(value as f64)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for Number {}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Add<f64> for Number {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Number(self.0 + rhs)
    }
}

impl Add<i64> for Number {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Number(self.0) + Number::from(rhs)
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
