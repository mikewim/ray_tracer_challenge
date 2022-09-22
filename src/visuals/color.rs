use std::ops;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color(red, green, blue)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Self(self.0 * _rhs.0, self.1 * _rhs.1, self.2 * _rhs.2)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
    }
}

#[cfg(test)]
use crate::utils::float_equal;

#[cfg(test)]
impl Color {
    pub fn equal(self, point: Self) -> bool {
        float_equal(self.0, point.0) && float_equal(self.1, point.1) && float_equal(self.2, point.2)
    }
}
