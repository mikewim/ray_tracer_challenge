use std::ops;

const MIN_PX_VAL: f64 = 0.0;
const MAX_PX_VAL: f64 = 255.0;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color(red, green, blue)
    }

    pub fn to_vec(self) -> [u8; 3] {
        normalize_color_for_pixel([self.0, self.1, self.2])
    }
}

fn normalize_color_for_pixel(values: [f64; 3]) -> [u8; 3] {
    let mut normalized_vals: [u8; 3] = [0, 0, 0];
    for (i, _) in values.iter().enumerate() {
        normalized_vals[i] = (values[i] * MAX_PX_VAL)
            .clamp(MIN_PX_VAL, MAX_PX_VAL)
            .round() as u8;
    }

    normalized_vals
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

impl ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        Self(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs)
    }
}

#[cfg(test)]
impl Color {
    pub fn equal(self, color: Self) -> bool {
        crate::utils::float_equal(self.0, color.0)
            && crate::utils::float_equal(self.1, color.1)
            && crate::utils::float_equal(self.2, color.2)
    }
}
