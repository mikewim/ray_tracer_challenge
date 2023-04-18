use std::f64::consts::PI;
use std::io::Error;

use crate::base_types::{Point, Vector};
use crate::matrices::Matrix;
use crate::visuals::{
    CheckerPattern, Color, GradientPattern, Material, Pattern, RingPattern, StripePattern,
};
use crate::world::{Camera, Light, Object, Plane, Sphere, World};

pub fn generate_plane(dir: &str) -> Result<(), Error> {
    let mut floor = Plane::default();
    floor.set_material(Material {
        color: Color::new(1.0, 0.9, 0.9),
        diffuse: 0.7,
        specular: 0.0,
        patterns: vec![
            Box::new(CheckerPattern::new(
                Color::new(1.0, 0.0, 0.0),
                Color::new(1.0, 0.9, 0.9),
                Some([
                    Box::new(CheckerPattern::new(
                        Color::new(0.5, 0.5, 0.0),
                        Color::new(0.0, 0.5, 0.5),
                        None,
                    )),
                    Box::new(StripePattern::new(
                        Color::new(0.5, 0.5, 0.0),
                        Color::new(0.0, 0.5, 0.5),
                        None,
                    )),
                ]),
            )),
            Box::new(StripePattern::new(
                Color::new(0.0, 0.0, 0.0),
                Color::new(1.0, 1.0, 1.0),
                None,
            )),
        ],
        reflective: 1.0,
        ..Material::default()
    });

    let mut ceil = Plane::default();
    ceil.set_material(Material {
        color: Color::new(0.0, 0.0, 1.0),
        diffuse: 0.7,
        specular: 0.0,
        reflective: 0.75,
        ..Material::default()
    });
    ceil.set_transform(Matrix::translation(0.0, 120.0, 0.0));

    let mut middle_sphere = Sphere::default();
    let mut stripe_pattern =
        StripePattern::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), None);
    stripe_pattern.set_transform(Matrix::scaling(0.25, 0.25, 0.25));
    middle_sphere.set_material(Material {
        color: Color::new(0.1, 1.0, 0.5),
        diffuse: 0.7,
        specular: 0.3,
        patterns: vec![Box::new(stripe_pattern)],
        reflective: 0.5,
        ..Material::default()
    });
    middle_sphere.set_transform(Matrix::translation(-0.5, 1.0, 0.5));

    let mut right_sphere = Sphere::default();
    let mut ring_pattern = RingPattern::new(
        Color::new(0.0, 0.0, 0.0),
        Color::new(1.0, 1.0, 1.0),
        Some([
            Box::new(CheckerPattern::new(
                Color::new(0.5, 0.15, 0.0),
                Color::new(0.0, 0.75, 0.15),
                None,
            )),
            Box::new(GradientPattern::new(
                Color::new(0.0, 0.5, 0.0),
                Color::new(0.0, 0.85, 0.5),
                None,
            )),
        ]),
    );
    ring_pattern.set_transform(Matrix::scaling(0.15, 0.15, 0.5));
    right_sphere.set_material(Material {
        color: Color::new(1.0, 0.0, 0.25),
        diffuse: 0.7,
        specular: 0.3,
        patterns: vec![Box::new(ring_pattern)],
        ..Material::default()
    });
    right_sphere
        .set_transform(Matrix::translation(1.5, 0.5, -0.5).mul(&Matrix::scaling(0.5, 0.5, 0.5)));

    let mut left_sphere = Sphere::default();
    let gradient_pattern =
        GradientPattern::new(Color::new(0.0, 1.0, 0.0), Color::new(1.0, 0.0, 1.0), None);
    // gradient_pattern.set_transform(Matrix::scaling(0.15, 0.15, 0.5));
    left_sphere.set_material(Material {
        color: Color::new(1.0, 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        patterns: vec![Box::new(gradient_pattern)],
        ..Material::default()
    });
    left_sphere.set_transform(
        Matrix::translation(-1.0, 1.0, -0.75)
            .mul(&Matrix::scaling(0.33, 0.33, 0.33))
            .mul(&Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
    );

    let mut world = World::new();
    world.objects.push(Box::new(floor));
    world.objects.push(Box::new(ceil));
    world.objects.push(Box::new(middle_sphere));
    world.objects.push(Box::new(right_sphere));
    world.objects.push(Box::new(left_sphere));

    world.lights = vec![
        Light::new(
            Point::new_point(-10.0, 10.0, -10.0),
            Color::new(0.8, 0.8, 0.8),
        ),
        // Light::new(
        //     Point::new_point(5.0, 2.0, -10.0),
        //     Color::new(0.5, 1.0, 0.25),
        // ),
    ];

    // let mut camera = Camera::new(250, 125, PI / 2.0);
    let mut camera = Camera::new(764, 528, PI / 2.0);
    camera.transform = Matrix::view_transform(
        Point::new_point(0.0, 1.5, -5.0),
        Point::new_point(0.0, 1.0, 0.0),
        Vector::new_vector(0.0, 1.0, 0.0),
    );

    camera
        .render(&world)
        .save_canvas(format!("{}/{}", dir, "plane.webp").as_str())
}
