use super::{Intersection, Material, Ray, Shape, WorldShape};
use crate::base_types::{Coordinates, Point, Vector};
use crate::matrices::Matrix;
use crate::utils::float_equal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub transform: Matrix,
    pub material: Material,
}

impl Plane {
    pub fn new(
        point_a: Point,
        point_b: Point,
        point_c: Point,
        transform: Matrix,
        material: Material,
    ) -> Self {
        Self {
            point_a,
            point_b,
            point_c,
            transform,
            material,
        }
    }
}

impl Shape for Plane {
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        // if the ray has no y direction, it is considered parallel with the plane
        if float_equal(ray.direction.1, 0.0) {
            return None;
        }

        Some(vec![Intersection {
            distance: -ray.origin.1 / ray.direction.1,
            object: WorldShape::Plane(*self),
        }])
    }

    fn normal_at(&self, _: Point) -> Vector {
        Coordinates::new_vector(0.0, 1.0, 0.0)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            point_a: Coordinates::new_point(0.0, 0.0, 0.0),
            point_b: Coordinates::new_point(1.0, 0.0, 0.0),
            point_c: Coordinates::new_point(0.0, 0.0, 1.0),
            transform: Matrix::new_identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Coordinates, Plane, Ray, Shape, WorldShape};

    #[test]
    fn normal() {
        let plane = Plane::default();

        assert_eq!(
            plane.normal_at(Coordinates::new_point(0.0, 0.0, 0.0)),
            Coordinates::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            plane.normal_at(Coordinates::new_point(10.0, 0.0, -20.0)),
            Coordinates::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            plane.normal_at(Coordinates::new_point(0.0, -1.0, 70.0)),
            Coordinates::new_vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn intersect_parallel() {
        let plane = Plane::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 10.0, 0.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        assert!(plane.intersect(ray).is_none());
    }

    #[test]
    fn intersect_parallel_coplanar() {
        let plane = Plane::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, 0.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        assert!(plane.intersect(ray).is_none());
    }

    #[test]
    fn intersect_from_above() {
        let plane = WorldShape::Plane(Plane::default());
        let ray = Ray::new(
            Coordinates::new_point(0.0, 1.0, 0.0),
            Coordinates::new_vector(0.0, -1.0, 0.0),
        );

        let intersections_opt = plane.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections[0].distance, 1.0);
        assert_eq!(intersections[0].object, plane);
    }

    #[test]
    fn intersect_from_below() {
        let plane = WorldShape::Plane(Plane::default());
        let ray = Ray::new(
            Coordinates::new_point(0.0, -1.0, 0.0),
            Coordinates::new_vector(0.0, 1.0, 0.0),
        );

        let intersections_opt = plane.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections[0].distance, 1.0);
        assert_eq!(intersections[0].object, plane);
    }
}
