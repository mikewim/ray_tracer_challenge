use std::ops;

mod intersection;
mod ray;
mod vector;

pub use intersection::*;
pub use ray::*;
pub use vector::*;

pub type Point = Coordinates;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Coordinates(pub f64, pub f64, pub f64, pub f64);

impl Coordinates {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 0.0)
    }
}

#[cfg(test)]
impl Coordinates {
    pub fn equal(self, point: Self) -> bool {
        crate::utils::float_equal(self.0, point.0)
            && crate::utils::float_equal(self.1, point.1)
            && crate::utils::float_equal(self.2, point.2)
            && crate::utils::float_equal(self.3, point.3)
    }
}

impl ops::Add for Coordinates {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        let w = self.3 + _rhs.3;
        // cannot add two points (points have w = 1)
        assert!(w <= 1.0);
        Self(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2, w)
    }
}

impl ops::Sub for Coordinates {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self(
            self.0 - _rhs.0,
            self.1 - _rhs.1,
            self.2 - _rhs.2,
            self.3 - _rhs.3,
        )
    }
}

impl ops::Neg for Coordinates {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl ops::Mul<f64> for Coordinates {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs, self.3 * _rhs)
    }
}

impl ops::Div<f64> for Coordinates {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        Self(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs, self.3 / _rhs)
    }
}
