use crate::base_types::{
    combine_intersections, hit_index, prepare_computations, Intersection, IntersectionDetails,
    Point, Ray,
};
use crate::matrices::Matrix;
use crate::visuals::{Color, Material};
use crate::world::{Light, Object, Plane, Sphere};

const MAX_REFLECT_DEPTH: usize = 5;

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: vec![Light::new(
                Point::new_point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();

        for object in self.objects.iter() {
            if let Some(object_intersections) = object.intersect(ray) {
                intersections = combine_intersections(intersections, object_intersections);
            }
        }

        intersections
    }

    pub fn color_at(&self, ray: Ray, reflect_depth: Option<usize>) -> Color {
        let intersections = self.intersect(ray);
        if !intersections.is_empty() {
            if let Some(hit_index) = hit_index(&intersections) {
                let comps = prepare_computations(hit_index, ray, intersections);
                let reflect_depth = reflect_depth.unwrap_or(MAX_REFLECT_DEPTH);

                return self.shade_hit(comps, reflect_depth);
            }
        }

        Color::new(0.0, 0.0, 0.0)
    }

    fn shade_hit(&self, intersection_details: IntersectionDetails, reflect_depth: usize) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for (i, light) in self.lights.iter().enumerate() {
            color = color
                + light.lighting(
                    intersection_details.intersection.object,
                    intersection_details.point,
                    intersection_details.eye_normal,
                    intersection_details.surface_normal,
                    self.is_shadowed(intersection_details.over_point, i),
                );
        }

        color + self.reflected_color(&intersection_details, reflect_depth)
    }

    fn is_shadowed(&self, point: Point, light_index: usize) -> bool {
        let point_to_light = point - self.lights[light_index].position;
        let distance = point_to_light.magnitude();
        let ray_to_light = Ray {
            origin: self.lights[light_index].position,
            direction: point_to_light.normalize(),
        };

        let intersections = self.intersect(ray_to_light);
        if !intersections.is_empty() {
            if let Some(hit_index) = hit_index(&intersections) {
                if intersections[hit_index].distance < distance {
                    return true;
                }
            }
        }

        false
    }

    fn reflected_color(
        &self,
        intersection_details: &IntersectionDetails,
        reflect_depth: usize,
    ) -> Color {
        let reflective_value = intersection_details
            .intersection
            .object
            .get_material()
            .reflective;
        if reflective_value == 0.0 || reflect_depth < 1 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(
            intersection_details.over_point,
            intersection_details.reflect_vector,
        );

        self.color_at(reflect_ray, Some(reflect_depth - 1)) * reflective_value
    }
}

impl Default for World {
    fn default() -> Self {
        let lights = vec![Light::new(
            Point::new_point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )];

        let mut sphere_1 = Sphere::default();
        sphere_1.set_material(Material::new(
            Color::new(0.8, 1.0, 0.6),
            Vec::new(),
            0.1,
            0.7,
            0.2,
            200.0,
            0.0,
            0.0,
            1.0,
        ));

        let mut sphere_2 = Sphere::default();
        sphere_2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let mut plane = Plane::default();
        let plane_material = Material {
            reflective: 0.5,
            ..Material::default()
        };
        plane.set_material(plane_material);
        plane.set_transform(Matrix::translation(0.0, -1.0, 0.0));

        Self {
            objects: vec![Box::new(sphere_1), Box::new(sphere_2), Box::new(plane)],
            lights,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base_types::Vector;

    #[test]
    fn default_world_intersect() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let intersections = world.intersect(ray);

        assert!(intersections.len() > 0);

        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].distance, 4.5);
        assert_eq!(intersections[2].distance, 5.5);
        assert_eq!(intersections[3].distance, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[0].intersect(ray).unwrap();
        let intersection_details = prepare_computations(0, ray, intersections);

        assert!(world
            .shade_hit(intersection_details, MAX_REFLECT_DEPTH)
            .equal(Color::new(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn shading_an_inside_intersection() {
        let mut world = World::default();
        world.lights = vec![Light::new(
            Point::new_point(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )];
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[1].intersect(ray).unwrap();
        // we want the 0.5 intersection, which would be second here
        let intersection_details = prepare_computations(1, ray, intersections);
        assert!(intersection_details.is_inside);

        assert!(world
            .shade_hit(intersection_details, MAX_REFLECT_DEPTH)
            .equal(Color::new(0.90498, 0.90498, 0.90498)));
    }

    #[test]
    fn shading_in_shadow() {
        let mut world = World::default();
        world.lights = vec![Light::new(
            Point::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )];
        world.objects[1].set_transform(Matrix::translation(0.0, 0.0, 10.0));
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[1].intersect(ray).unwrap();
        let intersection_details = prepare_computations(0, ray, intersections);
        assert!(world
            .shade_hit(intersection_details, MAX_REFLECT_DEPTH)
            .equal(Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn color_ray_misses() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );

        assert_eq!(world.color_at(ray, None), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_ray_hits() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        assert!(world
            .color_at(ray, None)
            .equal(Color::new(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn color_ray_hits_behind() {
        let mut world = World::default();

        let mut material_1 = world.objects[1].get_material();
        material_1.ambient = 1.0;
        world.objects[1].set_material(material_1);

        let mut material_0 = world.objects[0].get_material();
        material_0.ambient = 1.0;
        world.objects[0].set_material(material_0);

        world.objects[0].get_material().ambient = 1.0;
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.75),
            Vector::new_vector(0.0, 0.0, -1.0),
        );

        assert!(world
            .color_at(ray, None)
            .equal(world.objects[1].get_material().color));
    }

    #[test]
    fn is_shadowed_false() {
        let world = World::default();

        assert!(!world.is_shadowed(Point::new_point(0.0, 10.0, 0.0), 0));
    }

    #[test]
    fn behind_sphere_is_shadowed() {
        let world = World::default();

        assert!(world.is_shadowed(Point::new_point(10.0, -10.0, 10.0), 0));
    }

    #[test]
    fn sphere_not_between_light_and_point() {
        let world = World::default();

        assert!(!world.is_shadowed(Point::new_point(-20.0, 20.0, -20.0), 0));
    }

    #[test]
    fn point_in_between_object_and_light() {
        let world = World::default();

        assert!(!world.is_shadowed(Point::new_point(-2.0, 2.0, -2.0), 0));
    }

    #[test]
    fn reflect_color_for_non_reflective() {
        let mut world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let mut object_material = world.objects[1].get_material();
        object_material.ambient = 1.0;
        world.objects[1].set_material(object_material);

        let intersections = world.intersect(ray);
        let intersection_details = prepare_computations(0, ray, intersections);

        assert_eq!(
            world.reflected_color(&intersection_details, MAX_REFLECT_DEPTH),
            Color::new(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn reflect_color_works() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let intersections = vec![Intersection {
            distance: 2.0_f64.sqrt(),
            object: world.objects[2].as_ref(),
        }];

        let intersection_details = prepare_computations(0, ray, intersections);

        assert!(world
            .reflected_color(&intersection_details, MAX_REFLECT_DEPTH)
            .equal(Color::new(0.1903323, 0.2379154, 0.14274924)));
    }

    #[test]
    fn shade_hit_uses_refected_color() {
        let world = World::default();
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let intersection = Intersection {
            distance: 2.0_f64.sqrt(),
            object: world.objects[2].as_ref(),
        };
        let intersection_details = prepare_computations(0, ray, vec![intersection]);

        assert!(world
            .shade_hit(intersection_details, MAX_REFLECT_DEPTH)
            .equal(Color::new(0.8767577, 0.9243407, 0.8291746)));
    }

    #[test]
    fn avoid_infinite_reflect_calls() {
        let mut world = World::new();
        let upper_plane = Plane::new(
            Matrix::translation(0.0, 1.0, 0.0),
            Material::new(
                Color::new(0.8, 1.0, 0.6),
                Vec::new(),
                0.1,
                0.7,
                0.2,
                200.0,
                1.0,
                0.0,
                1.0,
            ),
        );

        let lower_plane = Plane::new(
            Matrix::translation(0.0, -1.0, 0.0),
            Material::new(
                Color::new(0.8, 1.0, 0.6),
                Vec::new(),
                0.1,
                0.7,
                0.2,
                200.0,
                1.0,
                0.0,
                1.0,
            ),
        );

        world.add_object(Box::new(upper_plane));
        world.add_object(Box::new(lower_plane));

        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );

        world.color_at(ray, None);
        assert!(true);
    }
}
