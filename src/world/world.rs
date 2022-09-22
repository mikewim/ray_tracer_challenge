use crate::base_types::{
    combine_intersections, hit, prepare_computations, Coordinates, Intersection,
    IntersectionDetails, Point, Ray,
};
use crate::matrices::Matrix;
use crate::shapes::{Shape, Sphere, WorldShape};
use crate::visuals::{lighting, Color, Light, Material};

pub struct World {
    pub objects: Vec<WorldShape>,
    pub light: Light,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: Light::new(
                Coordinates::new_point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let mut intersections = Vec::new();

        for object in self.objects.iter() {
            if let Some(object_intersections) = object.intersect(ray) {
                intersections = combine_intersections(intersections, object_intersections);
            }
        }

        if intersections.is_empty() {
            return None;
        }

        Some(intersections)
    }

    pub fn shade_hit(&self, intersection_details: IntersectionDetails) -> Color {
        lighting(
            intersection_details.intersection.object.get_material(),
            self.light,
            intersection_details.point,
            intersection_details.eye_normal,
            intersection_details.surface_normal,
            self.is_shadowed(intersection_details.over_point),
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = hit(intersections) {
                let comps = prepare_computations(&hit, ray);
                return self.shade_hit(comps);
            }
        }

        Color::new(0.0, 0.0, 0.0)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let point_to_light = point - self.light.position;
        let distance = point_to_light.magnitude();
        let ray_to_light = Ray {
            origin: self.light.position,
            direction: point_to_light.normalize(),
        };

        if let Some(intersections) = self.intersect(ray_to_light) {
            if let Some(hit) = hit(intersections) {
                if hit.distance < distance {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::new(
            Coordinates::new_point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let sphere_1 = Sphere {
            material: Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, None),
            ..Sphere::default()
        };
        let sphere_2 = Sphere {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Sphere::default()
        };

        Self {
            objects: vec![WorldShape::Sphere(sphere_1), WorldShape::Sphere(sphere_2)],
            light,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_world_intersect() {
        let world = World::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let intersections_opt = world.intersect(ray);

        assert!(intersections_opt.is_some());

        let intersections = intersections_opt.unwrap();
        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].distance, 4.5);
        assert_eq!(intersections[2].distance, 5.5);
        assert_eq!(intersections[3].distance, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[0].intersect(ray).unwrap();
        let intersection_details = prepare_computations(&intersections[0], ray);

        assert!(world
            .shade_hit(intersection_details)
            .equal(Color::new(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn shading_an_inside_intersection() {
        let mut world = World::default();
        world.light = Light::new(
            Coordinates::new_point(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, 0.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[1].intersect(ray).unwrap();
        // we want the 0.5 intersection, which would be second here
        let intersection_details = prepare_computations(&intersections[1], ray);
        assert!(intersection_details.is_inside);

        assert!(world
            .shade_hit(intersection_details)
            .equal(Color::new(0.90498, 0.90498, 0.90498)));
    }

    #[test]
    fn shading_in_shadow() {
        let mut world = World::default();
        world.light = Light::new(
            Coordinates::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        world.objects[1].set_transform(Matrix::translation(0.0, 0.0, 10.0));
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, 5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        let intersections = world.objects[1].intersect(ray).unwrap();
        let intersection_details = prepare_computations(&intersections[0], ray);
        assert!(world
            .shade_hit(intersection_details)
            .equal(Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn color_ray_misses() {
        let world = World::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 1.0, 0.0),
        );

        assert_eq!(world.color_at(ray), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_ray_hits() {
        let world = World::default();
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );

        assert!(world
            .color_at(ray)
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
            Coordinates::new_point(0.0, 0.0, 0.75),
            Coordinates::new_vector(0.0, 0.0, -1.0),
        );

        assert!(world
            .color_at(ray)
            .equal(world.objects[1].get_material().color));
    }

    #[test]
    fn is_shadowed_false() {
        let world = World::default();

        assert!(!world.is_shadowed(Coordinates::new_point(0.0, 10.0, 0.0)));
    }

    #[test]
    fn behind_sphere_is_shadowed() {
        let world = World::default();

        assert!(world.is_shadowed(Coordinates::new_point(10.0, -10.0, 10.0)));
    }

    #[test]
    fn sphere_not_between_light_and_point() {
        let world = World::default();

        assert!(!world.is_shadowed(Coordinates::new_point(-20.0, 20.0, -20.0)));
    }

    #[test]
    fn point_in_between_object_and_light() {
        let world = World::default();

        assert!(!world.is_shadowed(Coordinates::new_point(-2.0, 2.0, -2.0)));
    }
}
