#[cfg(test)]
use std::any::Any;

mod light;
mod plane;
mod sphere;

pub use light::*;
pub use plane::*;
pub use sphere::*;

use crate::base_types::{Intersection, Point, Ray, Vector};
use crate::matrices::Matrix;
use crate::visuals::{Color, Material};

pub trait Object {
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn get_material(&self) -> Material;
    fn get_material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, material: Material);
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;
    fn color_at(&self, point: Point) -> Color {
        let object_space_point = self.get_transform().inverse().unwrap().coords_mul(point);
        self.get_material().color_at(object_space_point)
    }
    // these two functions have default functionality that will be the same for all objects
    fn normal_at(&self, point: Point) -> Vector {
        // if inverse of transform does not exist, we should panic here
        let transform_inverse = self.get_transform().inverse().unwrap();
        let object_point = transform_inverse.coords_mul(point);
        let object_normal = self.local_normal_at(object_point);
        let mut world_normal = transform_inverse.transpose().coords_mul(object_normal);
        // hack to reset to vector, i.e., the above calc set w to 1 but we want it to be 0
        world_normal.w = 0.0;

        world_normal.normalize()
    }
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        // every shape will need to transform the ray first
        let transform_inverse = self.get_transform().inverse()?;
        let transformed_ray = ray.transform(&transform_inverse);
        self.local_intersect(transformed_ray)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any;
    #[cfg(test)]
    fn equal(&self, object: &dyn Object) -> bool;
}
