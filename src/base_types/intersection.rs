use super::{Point, Ray, Vector};
use crate::utils::{combine_sorted, is_sorted, FLOAT_DIFF};
use crate::world::Object;

pub struct IntersectionDetails<'a> {
    pub intersection: Intersection<'a>,
    pub point: Point,
    pub eye_normal: Vector,
    pub surface_normal: Vector,
    pub is_inside: bool,
    pub reflect_vector: Vector,
    pub refractive_entry_index: f64,
    pub refractive_exit_index: f64,
    // used for shading to adjust the point
    // under consideration up just a bit
    // in the direction of the normal.
    // This is necessary due to the imprecision
    // of floating point ops and sometimes
    // the is_shadowed function taking
    // the sphere surface as a point
    // that needs to be shadowed
    pub over_point: Point,
}

#[derive(Clone)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a dyn Object,
}

#[cfg(test)]
impl<'a> Intersection<'a> {
    pub fn equal(&self, intersection: &Self) -> bool {
        self.distance == intersection.distance && std::ptr::eq(self.object, intersection.object)
    }
}

// consumes the two params and returns new vector of intersections in sorted
// order
pub fn combine_intersections<'a>(
    mut vec1: Vec<Intersection<'a>>,
    vec2: Vec<Intersection<'a>>,
) -> Vec<Intersection<'a>> {
    let separator = vec1.len();
    vec1.extend(vec2);
    combine_sorted(&mut vec1, separator);

    vec1
}

// **Note this expects a sorted set of intersections!
// returns the index of the first objec the ray encounters
pub fn hit_index(intersections: &Vec<Intersection>) -> Option<usize> {
    assert!(is_sorted(intersections));

    for (i, intersection) in intersections.iter().enumerate() {
        if intersection.distance >= 0.0 {
            return Some(i);
        }
    }

    None
}

pub fn prepare_computations(
    hit_index: usize,
    ray: Ray,
    intersections: Vec<Intersection>,
) -> IntersectionDetails {
    let point = ray.position(intersections[hit_index].distance);
    let mut surface_normal = intersections[hit_index].object.normal_at(point);
    let eye_normal = -ray.direction;
    let is_inside: bool;

    if eye_normal.dot(surface_normal) < 0.0 {
        is_inside = true;
        surface_normal = -surface_normal;
    } else {
        is_inside = false;
    }

    IntersectionDetails {
        intersection: intersections[hit_index].clone(),
        point,
        eye_normal,
        surface_normal,
        is_inside,
        over_point: point + surface_normal * FLOAT_DIFF,
        reflect_vector: ray.direction.reflect(surface_normal),
        refractive_exit_index: 1.0,
        refractive_entry_index: 1.0,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        base_types::intersection,
        world::{Plane, Sphere},
    };

    #[test]
    fn hit_test() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection {
                distance: -3.0,
                object: &sphere,
            },
            Intersection {
                distance: 2.0,
                object: &sphere,
            },
            Intersection {
                distance: 5.0,
                object: &sphere,
            },
            Intersection {
                distance: 7.0,
                object: &sphere,
            },
        ];

        let hit_index = hit_index(&intersections);
        assert!(hit_index.is_some());
        assert_eq!(intersections[hit_index.unwrap()].distance, 2.0);
    }

    #[test]
    fn prepare_computations_test() {
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();
        let intersections = sphere.intersect(ray).unwrap();
        let hit_index = hit_index(&intersections).unwrap();

        let intersection = intersections[hit_index].clone();
        let intersection_details = prepare_computations(hit_index, ray, intersections);
        assert!(intersection_details.intersection.equal(&intersection));
        assert_eq!(intersection_details.point, Point::new_point(0.0, 0.0, -1.0));
        assert_eq!(
            intersection_details.eye_normal,
            Vector::new_vector(0.0, 0.0, -1.0)
        );
        assert_eq!(
            intersection_details.surface_normal,
            Vector::new_vector(0.0, 0.0, -1.0)
        );
        assert!(!intersection_details.is_inside);
    }

    #[test]
    fn prepare_computations_inside_test() {
        let ray = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();
        let intersections = sphere.intersect(ray).unwrap();
        let hit_index = hit_index(&intersections).unwrap();

        let intersection = intersections[hit_index].clone();
        let intersection_details = prepare_computations(hit_index, ray, intersections);
        assert!(intersection_details.intersection.equal(&intersection));
        assert_eq!(intersection_details.point, Point::new_point(0.0, 0.0, 1.0));
        assert_eq!(
            intersection_details.eye_normal,
            Vector::new_vector(0.0, 0.0, -1.0)
        );
        assert_eq!(
            intersection_details.surface_normal,
            Vector::new_vector(0.0, 0.0, -1.0)
        );
        assert!(intersection_details.is_inside);
    }

    #[test]
    fn prepare_computations_reflect_test() {
        let ray = Ray::new(
            Point::new_point(0.0, 1.0, -1.0),
            Vector::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let shape = Plane::default();
        let intersections = vec![Intersection {
            distance: 2.0_f64.sqrt(),
            object: &shape,
        }];
        let hit_index = hit_index(&intersections).unwrap();

        let intersection = intersections[hit_index].clone();
        let intersection_details = prepare_computations(hit_index, ray, intersections);
        assert!(intersection_details.intersection.equal(&intersection));
        assert_eq!(
            intersection_details.reflect_vector,
            Vector::new_vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }
}
