use crate::base_types::{Point, Vector};
use crate::matrices::Matrix;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, matrix: &Matrix) -> Self {
        Self {
            origin: matrix.coords_mul(self.origin),
            direction: matrix.coords_mul(self.direction),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position() {
        let ray = Ray::new(
            Point::new_point(2.0, 3.0, 4.0),
            Vector::new_vector(1.0, 0.0, 0.0),
        );

        assert_eq!(ray.position(0.0), Point::new_point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new_point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate() {
        let ray = Ray::new(
            Point::new_point(1.0, 2.0, 3.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let matrix = Matrix::translation(3.0, 4.0, 5.0);
        let ray2 = ray.transform(&matrix);

        assert_eq!(ray2.origin, Point::new_point(4.0, 6.0, 8.0));
        assert_eq!(ray2.direction, Vector::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale() {
        let ray = Ray::new(
            Point::new_point(1.0, 2.0, 3.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let ray2 = ray.transform(&matrix);

        assert_eq!(ray2.origin, Point::new_point(2.0, 6.0, 12.0));
        assert_eq!(ray2.direction, Vector::new_vector(0.0, 3.0, 0.0));
    }
}
