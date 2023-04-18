use super::Object;
use crate::base_types::{Intersection, Point, Ray, Vector};
use crate::matrices::Matrix;
use crate::utils;
use crate::visuals::Material;

#[cfg(test)]
use std::any::Any;

// will treat the plane as a plane on the x and z axes that goes through the origin.
// any movement can be had through transformations
#[cfg_attr(test, derive(PartialEq))]
pub struct Plane {
    transform: Matrix,
    material: Material,
}

impl Plane {
    pub fn new(transform: Matrix, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }
}

impl Object for Plane {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        // if parallel or coplanar, no intersections
        if utils::float_equal(ray.direction.y, 0.0) {
            return None;
        }

        let distance = -ray.origin.y / ray.direction.y;
        Some(vec![Intersection {
            distance,
            object: self,
        }])
    }

    fn local_normal_at(&self, _: Point) -> Vector {
        Vector::new_vector(0.0, 1.0, 0.0)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[cfg(test)]
    fn equal(&self, object: &dyn Object) -> bool {
        match object.as_any().downcast_ref::<Plane>() {
            Some(plane) => *self == *plane,
            None => false,
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix::new_identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Object, Plane};
    use crate::base_types::{Point, Ray, Vector};

    #[test]
    fn normal() {
        let plane = Plane::default();

        assert_eq!(
            plane.normal_at(Point::new_point(0.0, 0.0, 0.0)),
            Vector::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            plane.normal_at(Point::new_point(10.0, 0.0, -20.0)),
            Vector::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            plane.normal_at(Point::new_point(0.0, -1.0, 70.0)),
            Vector::new_vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn intersect_parallel() {
        let plane = Plane::default();
        let ray = Ray::new(
            Point::new_point(0.0, 10.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        assert!(plane.intersect(ray).is_none());
    }

    #[test]
    fn intersect_parallel_coplanar() {
        let plane = Plane::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        assert!(plane.intersect(ray).is_none());
    }

    #[test]
    fn intersect_from_above() {
        let plane = Plane::default();
        let ray = Ray::new(
            Point::new_point(0.0, 1.0, 0.0),
            Vector::new_vector(0.0, -1.0, 0.0),
        );

        let intersections_opt = plane.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections[0].distance, 1.0);
        assert!(intersections[0].object.equal(&plane));
    }

    #[test]
    fn intersect_from_below() {
        let plane = Plane::default();
        let ray = Ray::new(
            Point::new_point(0.0, -1.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );

        let intersections_opt = plane.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections[0].distance, 1.0);
        assert!(intersections[0].object.equal(&plane));
    }
}
