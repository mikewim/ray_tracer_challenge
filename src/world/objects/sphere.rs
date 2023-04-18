use super::{Intersection, Material, Object, Ray};
use crate::base_types::{Point, Vector};
use crate::matrices::Matrix;

#[cfg(test)]
use std::any::Any;

#[cfg_attr(test, derive(PartialEq))]
pub struct Sphere {
    center: Point,
    radius: f64,
    transform: Matrix,
    material: Material,
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

impl Object for Sphere {
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

        // sort ascending
        if intersection1 <= intersection2 {
            return Some(vec![
                Intersection {
                    distance: intersection1,
                    object: self,
                },
                Intersection {
                    distance: intersection2,
                    object: self,
                },
            ]);
        }

        Some(vec![
            Intersection {
                distance: intersection2,
                object: self,
            },
            Intersection {
                distance: intersection1,
                object: self,
            },
        ])
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - self.center
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[cfg(test)]
    fn equal(&self, object: &dyn Object) -> bool {
        match object.as_any().downcast_ref::<Sphere>() {
            Some(sphere) => *sphere == *self,
            None => false,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::new_identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::visuals::{Color, Pattern, StripePattern};
    use std::f64::consts::PI;

    #[test]
    fn normal() {
        let sphere = Sphere::default();

        assert_eq!(
            sphere.normal_at(Point::new_point(1.0, 0.0, 0.0)),
            Vector::new_vector(1.0, 0.0, 0.0)
        );

        assert_eq!(
            sphere.normal_at(Point::new_point(0.0, 1.0, 0.0)),
            Vector::new_vector(0.0, 1.0, 0.0)
        );

        assert_eq!(
            sphere.normal_at(Point::new_point(0.0, 0.0, 1.0)),
            Vector::new_vector(0.0, 0.0, 1.0)
        );

        let three: f64 = 3.0;
        assert_eq!(
            sphere.normal_at(Point::new_point(
                three.sqrt() / three,
                three.sqrt() / three,
                three.sqrt() / three
            )),
            Vector::new_vector(
                three.sqrt() / three,
                three.sqrt() / three,
                three.sqrt() / three
            )
        );
    }

    #[test]
    fn normal_with_transform() {
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::translation(0.0, 1.0, 0.0));

        assert!(sphere
            .normal_at(Point::new_point(0.0, 1.70711, -0.707111))
            .equal(Vector::new_vector(0.0, 0.707111, -0.707111)));

        sphere.set_transform(Matrix::scaling(1.0, 0.5, 1.0).mul(&Matrix::rotation_z(PI / 5.0)));

        assert!(sphere
            .normal_at(Point::new_point(
                0.0,
                (2.0 as f64).sqrt() / 2.0,
                -(2.0 as f64).sqrt() / 2.0
            ))
            .equal(Vector::new_vector(0.0, 0.97014, -0.24254)));
    }

    #[test]
    fn intersect() {
        let ray = Ray::new(
            Point::new_point(400.0, 250.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = Sphere::default();

        sphere.set_transform(Matrix::translation(400.0, 250.0, 0.0));

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 4.0);
        assert!(intersections[0].object.equal(&sphere));
        assert_eq!(intersections[1].distance, 6.0);
        assert!(intersections[1].object.equal(&sphere));
    }

    #[test]
    fn intersect_tangent() {
        let ray = Ray::new(
            Point::new_point(0.0, 1.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = &Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 5.0);
        assert!(intersections[0].object.equal(sphere));
        assert_eq!(intersections[1].distance, 5.0);
        assert!(intersections[1].object.equal(sphere));
    }

    #[test]
    fn intersect_miss() {
        let ray = Ray::new(
            Point::new_point(0.0, 2.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_none());
    }

    #[test]
    fn intersect_middle() {
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = &Sphere::default();

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, -1.0);
        assert!(intersections[0].object.equal(sphere));
        assert_eq!(intersections[1].distance, 1.0);
        assert!(intersections[1].object.equal(sphere));
    }

    #[test]
    fn intersect_scaled() {
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 3.0);
        assert!(intersections[0].object.equal(&sphere));
        assert_eq!(intersections[1].distance, 7.0);
        assert!(intersections[1].object.equal(&sphere));
    }

    #[test]
    fn intersect_translate_miss() {
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::translation(5.0, 0.0, 0.0));

        let intersections_opt = sphere.intersect(ray);
        assert!(intersections_opt.is_none());
    }

    #[test]
    fn color_at_with_object_transform() {
        let color_a = Color::new(0.0, 0.0, 0.0);
        let color_b = Color::new(1.0, 1.0, 1.0);
        let pattern = StripePattern::new(color_a, color_b, None);

        let mut sphere = Sphere::default();

        let mut material = Material::default();
        material.patterns = vec![Box::new(pattern)];
        sphere.set_material(material);

        sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        assert_eq!(sphere.color_at(Point::new_point(1.5, 0.0, 0.0)), color_a);
        assert_eq!(sphere.color_at(Point::new_point(3.5, 0.0, 0.0)), color_b);
    }

    #[test]
    fn color_at_with_pattern_transform() {
        let color_a = Color::new(0.0, 0.0, 0.0);
        let color_b = Color::new(1.0, 1.0, 1.0);
        let mut pattern = StripePattern::new(color_a, color_b, None);
        pattern.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let mut sphere = Sphere::default();

        let mut material = Material::default();
        material.patterns = vec![Box::new(pattern)];
        sphere.set_material(material);

        assert_eq!(sphere.color_at(Point::new_point(1.5, 0.0, 0.0)), color_a);
        assert_eq!(sphere.color_at(Point::new_point(3.5, 0.0, 0.0)), color_b);
    }

    #[test]
    fn color_at_with_pattern_and_object_transform() {
        let color_a = Color::new(0.0, 0.0, 0.0);
        let color_b = Color::new(1.0, 1.0, 1.0);
        let mut pattern = StripePattern::new(color_a, color_b, None);
        pattern.set_transform(Matrix::translation(0.5, 0.0, 0.0));

        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let mut material = Material::default();
        material.patterns = vec![Box::new(pattern)];
        sphere.set_material(material);

        assert_eq!(sphere.color_at(Point::new_point(2.5, 0.0, 0.0)), color_a);
        assert_eq!(sphere.color_at(Point::new_point(3.5, 0.0, 0.0)), color_b);
    }
}
