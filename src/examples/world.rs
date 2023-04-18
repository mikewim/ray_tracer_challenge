use std::f64::consts::PI;
use std::io::Error;

use crate::base_types::{Point, Vector};
use crate::matrices::Matrix;
use crate::visuals::{Color, Material};
use crate::world::{Camera, Light, Object, Sphere, World};

pub fn generate_world(dir: &str) -> Result<(), Error> {
    let floor_material = Material {
        color: Color::new(1.0, 0.9, 0.9),
        diffuse: 0.7,
        specular: 0.0,
        ..Material::default()
    };
    // floor is sphere that is flattened with matte texture
    let mut floor = Sphere::default();
    floor.set_material(floor_material.clone());
    floor
        .set_transform(Matrix::scaling(10.0, 0.01, 10.0).mul(&Matrix::translation(0.0, -1.0, 0.0)));

    // left_wall is sphere that is rotated into place
    let mut left_wall = Sphere::default();
    left_wall.set_material(floor_material.clone());
    left_wall.set_transform(
        Matrix::translation(0.0, 0.0, 5.0)
            .mul(&Matrix::rotation_y(-PI / 4.0))
            .mul(&Matrix::rotation_x(PI / 2.0))
            .mul(&Matrix::scaling(10.0, 0.01, 10.0)),
    );

    // right_wall is sphere that is rotated into place
    let mut right_wall = Sphere::default();
    right_wall.set_material(floor_material);
    right_wall.set_transform(
        Matrix::translation(0.0, 0.0, 5.0)
            .mul(&Matrix::rotation_y(PI / 4.0))
            .mul(&Matrix::rotation_x(PI / 2.0))
            .mul(&Matrix::scaling(10.0, 0.01, 10.0)),
    );

    let mut middle_sphere = Sphere::default();
    middle_sphere.set_material(Material {
        color: Color::new(0.1, 1.0, 0.5),
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    });
    middle_sphere.set_transform(Matrix::translation(-0.5, 1.0, 0.5));

    let mut right_sphere = Sphere::default();
    right_sphere.set_material(Material {
        color: Color::new(0.5, 1.0, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    });
    right_sphere
        .set_transform(Matrix::translation(1.5, 0.5, -0.5).mul(&Matrix::scaling(0.5, 0.5, 0.5)));

    let mut left_sphere = Sphere::default();
    left_sphere.set_material(Material {
        color: Color::new(1.0, 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    });
    left_sphere.set_transform(
        Matrix::translation(-1.0, 1.0, -0.75)
            .mul(&Matrix::scaling(0.33, 0.33, 0.33))
            .mul(&Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
    );

    let mut world = World::new();
    world.objects.push(Box::new(floor));
    world.objects.push(Box::new(left_wall));
    world.objects.push(Box::new(right_wall));
    world.objects.push(Box::new(middle_sphere));
    world.objects.push(Box::new(right_sphere));
    world.objects.push(Box::new(left_sphere));

    world.lights = vec![
        Light::new(
            Point::new_point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ),
        Light::new(
            Point::new_point(5.0, 2.0, -10.0),
            Color::new(0.5, 1.0, 0.25),
        ),
    ];

    // let mut camera = Camera::new(250, 125, PI / 3.0);
    let mut camera = Camera::new(764, 528, PI / 3.0);
    camera.transform = Matrix::view_transform(
        Point::new_point(0.0, 1.5, -5.0),
        Point::new_point(0.0, 1.0, 0.0),
        Vector::new_vector(0.0, 1.0, 0.0),
    );

    camera
        .render(&world)
        .save_canvas(format!("{}/{}", dir, "world.webp").as_str())
}
