use super::Coordinates;

pub type Point = Coordinates;

impl Point {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}
