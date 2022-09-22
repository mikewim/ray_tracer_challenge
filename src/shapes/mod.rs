mod plane;
mod sphere;

pub use plane::*;
pub use sphere::*;

use crate::base_types::{Intersection, Point, Ray, Vector};
use crate::matrices::Matrix;
use crate::visuals::Material;

// This is used to represent objects in the world or intersection.
// It's in enum format to basically ensure it has a consistent size
// across every shape and allows storing these shapes as vectors
// in the world to be acted upon.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorldShape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape for WorldShape {
    fn get_transform(&self) -> Matrix {
        match self {
            WorldShape::Sphere(sphere) => sphere.get_transform(),
            WorldShape::Plane(plane) => plane.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: Matrix) {
        match self {
            WorldShape::Sphere(val) => val.set_transform(transform),
            WorldShape::Plane(plane) => plane.set_transform(transform),
        }
    }

    fn get_material(&self) -> Material {
        match self {
            WorldShape::Sphere(val) => val.get_material(),
            WorldShape::Plane(plane) => plane.get_material(),
        }
    }

    fn set_material(&mut self, material: Material) {
        match self {
            WorldShape::Sphere(val) => val.set_material(material),
            WorldShape::Plane(plane) => plane.set_material(material),
        }
    }

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        // every shape will need to transform the ray first
        let transform_inverse = self.get_transform().inverse()?;
        let transformed_ray = ray.transform(transform_inverse);

        match self {
            WorldShape::Sphere(sphere) => sphere.intersect(transformed_ray),
            WorldShape::Plane(plane) => plane.intersect(transformed_ray),
        }
    }

    fn normal_at(&self, point: Point) -> Vector {
        // transform point to object space first
        let transform_inverse = match self {
            WorldShape::Sphere(sphere) => sphere.get_transform().inverse().unwrap(),
            WorldShape::Plane(plane) => plane.get_transform().inverse().unwrap(),
        };
        let object_point = transform_inverse * point;
        let object_normal = match self {
            WorldShape::Sphere(sphere) => sphere.normal_at(object_point),
            WorldShape::Plane(plane) => plane.normal_at(object_point),
        };

        let mut world_normal = transform_inverse.transpose() * object_normal;
        // hack to reset to vector, i.e., the above calc set w to 1 but we want it to be 0
        world_normal.3 = 0.0;

        world_normal.normalize()
    }
}

pub trait Shape {
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;
    fn normal_at(&self, point: Point) -> Vector;
}
