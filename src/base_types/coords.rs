use std::ops;

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl ops::Add for Coordinates {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        let w = self.w + _rhs.w;
        if w > 1.0 {
            panic!("Cannot add two points together!");
        }

        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w,
        }
    }
}

impl ops::Add<f64> for Coordinates {
    type Output = Self;

    fn add(self, _rhs: f64) -> Self {
        Self {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
            w: self.w,
        }
    }
}

impl ops::Sub for Coordinates {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        let w = self.w - _rhs.w;
        if w < 0.0 {
            panic!("Cannot subtract a point from a vector!");
        }
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w,
        }
    }
}

impl ops::Neg for Coordinates {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl ops::Mul<f64> for Coordinates {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w,
        }
    }
}

impl ops::Div<f64> for Coordinates {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w,
        }
    }
}

#[cfg(test)]
impl Coordinates {
    pub fn equal(self: Self, rhs: Self) -> bool {
        crate::utils::float_equal(self.x, rhs.x)
            && crate::utils::float_equal(self.y, rhs.y)
            && crate::utils::float_equal(self.z, rhs.z)
            && crate::utils::float_equal(self.w, rhs.w)
    }
}
