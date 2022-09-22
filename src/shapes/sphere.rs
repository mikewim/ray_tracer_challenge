use super::{Intersection, Material, Ray, Shape, WorldShape};
use crate::base_types::{Coordinates, Point, Vector};
use crate::matrices::Matrix;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, transform: Matrix, material: Material) -> Self {
        Self {
            center,
            radius,
            transform,
            material,
        }
    }
}

impl Shape for Sphere {
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
        let sphere_to_ray = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - (self.radius * self.radius);

        let discriminant = b.powi(2) - (4.0 * a * c);

        // there was no intersection
        if discriminant < 0.0 {
            return None;
        }

        let component1 = discriminant.sqrt();
        let component2 = 2.0 * a;
        let intersection1 = (-b - component1) / component2;
        let intersection2 = (-b + component1) / component2;
        let world_sphere = WorldShape::Sphere(*self);

        // sort ascending
        if intersection1 <= intersection2 {
            return Some(vec![
                Intersection {
                    distance: intersection1,
                    object: world_sphere,
                },
                Intersection {
                    distance: intersection2,
                    object: world_sphere,
                },
            ]);
        }

        Some(vec![
            Intersection {
                distance: intersection2,
                object: world_sphere,
            },
            Intersection {
                distance: intersection1,
                object: world_sphere,
            },
        ])
    }

    fn normal_at(&self, point: Point) -> Vector {
        point - self.center
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Coordinates::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::new_identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn normal() {
        let sphere = Sphere::default();

        assert_eq!(
            sphere.normal_at(Coordinates::new_point(1.0, 0.0, 0.0)),
            Coordinates::new_vector(1.0, 0.0, 0.0)
        );

        assert_eq!(
            sphere.normal_at(Coordinates::new_point(0.0, 1.0, 0.0)),
            Coordinates::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            sphere.normal_at(Coordinates::new_point(0.0, 0.0, 1.0)),
            Coordinates::new_vector(0.0, 0.0, 1.0)
        );

        let three: f64 = 3.0;
        assert_eq!(
            sphere.normal_at(Coordinates::new_point(
                three.sqrt() / three,
                three.sqrt() / three,
                three.sqrt() / three
            )),
            Coordinates::new_vector(
                three.sqrt() / three,
                three.sqrt() / three,
                three.sqrt() / three
            )
        );
    }

    #[test]
    fn normal_with_transform() {
        let mut sphere = WorldShape::Sphere(Sphere::default());
        sphere.set_transform(Matrix::translation(0.0, 1.0, 0.0));

        assert!(sphere
            .normal_at(Coordinates::new_point(0.0, 1.70711, -0.707111))
            .equal(Coordinates::new_vector(0.0, 0.707111, -0.707111)));

        sphere.set_transform(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0));

        assert!(sphere
            .normal_at(Coordinates::new_point(
                0.0,
                (2.0 as f64).sqrt() / 2.0,
                -(2.0 as f64).sqrt() / 2.0
            ))
            .equal(Coordinates::new_vector(0.0, 0.97014, -0.24254)));
    }

    #[test]
    fn intersect() {
        let ray = Ray::new(
            Coordinates::new_point(400.0, 250.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = Sphere::default();
        sphere.radius = 1.0;
        sphere.center = Coordinates::new_point(400.0, 250.0, 0.0);

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 4.0);

        match intersections[0].object {
            WorldShape::Sphere(val) => assert_eq!(val, sphere),
            _ => assert!(false),
        }
        assert_eq!(intersections[1].distance, 6.0);
        match intersections[1].object {
            WorldShape::Sphere(val) => assert_eq!(val, sphere),
            _ => assert!(false),
        }
    }

    #[test]
    fn intersect_tangent() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 1.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());
        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 5.0);
        assert_eq!(intersections[0].object, WorldShape::Sphere(sphere));
        assert_eq!(intersections[1].distance, 5.0);
        assert_eq!(intersections[1].object, WorldShape::Sphere(sphere));
    }

    #[test]
    fn intersect_miss() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 2.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_none());
    }

    #[test]
    fn intersect_middle() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, 0.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());
        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, -1.0);
        assert_eq!(intersections[0].object, WorldShape::Sphere(sphere));
        assert_eq!(intersections[1].distance, 1.0);
        assert_eq!(intersections[1].object, WorldShape::Sphere(sphere));
    }

    #[test]
    fn intersect_scaled() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = WorldShape::Sphere(Sphere::default());
        sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());
        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 3.0);
        assert_eq!(intersections[0].object, sphere);
        assert_eq!(intersections[1].distance, 7.0);
        assert_eq!(intersections[1].object, sphere);
    }

    #[test]
    fn intersect_translate() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = WorldShape::Sphere(Sphere::default());
        sphere.set_transform(Matrix::translation(5.0, 0.0, 0.0));

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_none());
    }
}
