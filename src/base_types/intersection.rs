use crate::base_types::{Point, Ray, Vector};
use crate::shapes::{Shape, WorldShape};
use crate::utils::{combine_sorted, is_sorted, FLOAT_DIFF};

pub struct IntersectionDetails<'a> {
    pub intersection: &'a Intersection,
    pub point: Point,
    pub eye_normal: Vector,
    pub surface_normal: Vector,
    pub is_inside: bool,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub distance: f64,
    pub object: WorldShape,
}

// consumes the two params and returns new vector of intersections in sorted
// order
pub fn combine_intersections(
    mut vec1: Vec<Intersection>,
    vec2: Vec<Intersection>,
) -> Vec<Intersection> {
    let separator = vec1.len();
    vec1.extend(vec2);
    combine_sorted(&mut vec1, separator);

    vec1
}

// **Note this expects a sorted set of intersections!
pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    assert!(is_sorted(&intersections));
    let positive_intersections: Vec<Intersection> = intersections
        .into_iter()
        .filter(|intersection| intersection.distance > 0.0)
        .collect();

    if positive_intersections.is_empty() {
        return None;
    }

    // first lowest, non-negative intersections is returned
    Some(positive_intersections[0])
}

pub fn prepare_computations(intersection: &Intersection, ray: Ray) -> IntersectionDetails {
    let point = ray.position(intersection.distance);
    let mut surface_normal = intersection.object.normal_at(point);
    let eye_normal = -ray.direction;
    let is_inside: bool;

    if eye_normal.dot(surface_normal) < 0.0 {
        is_inside = true;
        surface_normal = -surface_normal;
    } else {
        is_inside = false;
    }

    IntersectionDetails {
        intersection,
        point,
        eye_normal,
        surface_normal,
        is_inside,
        over_point: point + surface_normal * FLOAT_DIFF,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base_types::Coordinates;
    use crate::shapes::Sphere;

    #[test]
    fn hit_test() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection {
                distance: -3.0,
                object: WorldShape::Sphere(sphere),
            },
            Intersection {
                distance: 2.0,
                object: WorldShape::Sphere(sphere),
            },
            Intersection {
                distance: 5.0,
                object: WorldShape::Sphere(sphere),
            },
            Intersection {
                distance: 7.0,
                object: WorldShape::Sphere(sphere),
            },
        ];

        let hit = hit(intersections);
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().distance, 2.0);
    }

    #[test]
    fn prepare_computations_test() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, -5.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();
        let intersections = sphere.intersect(ray).unwrap();
        let hit = hit(intersections).unwrap();

        let intersection_details = prepare_computations(&hit, ray);
        assert_eq!(*intersection_details.intersection, hit);
        assert_eq!(
            intersection_details.intersection.object,
            WorldShape::Sphere(sphere)
        );
        assert_eq!(
            intersection_details.point,
            Coordinates::new_point(0.0, 0.0, -1.0)
        );
        assert_eq!(
            intersection_details.eye_normal,
            Coordinates::new_vector(0.0, 0.0, -1.0)
        );
        assert_eq!(
            intersection_details.surface_normal,
            Coordinates::new_vector(0.0, 0.0, -1.0)
        );
        assert!(!intersection_details.is_inside);
    }

    #[test]
    fn prepare_computations_inside_test() {
        let ray = Ray::new(
            Coordinates::new_point(0.0, 0.0, 0.0),
            Coordinates::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::default();
        let intersections = sphere.intersect(ray).unwrap();
        let hit = hit(intersections).unwrap();

        let intersection_details = prepare_computations(&hit, ray);
        assert_eq!(*intersection_details.intersection, hit);
        assert_eq!(
            intersection_details.intersection.object,
            WorldShape::Sphere(sphere)
        );
        assert_eq!(
            intersection_details.point,
            Coordinates::new_point(0.0, 0.0, 1.0)
        );
        assert_eq!(
            intersection_details.eye_normal,
            Coordinates::new_vector(0.0, 0.0, -1.0)
        );
        assert_eq!(
            intersection_details.surface_normal,
            Coordinates::new_vector(0.0, 0.0, -1.0)
        );
        assert!(intersection_details.is_inside);
    }
}
