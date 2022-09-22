use std::f64::consts::PI;
use std::fs::write;
use std::io::Error;

use crate::base_types::Coordinates;
use crate::matrices::Matrix;
use crate::shapes::{Sphere, WorldShape};
use crate::visuals::{Color, Light, Material};
use crate::world::{Camera, World};

pub fn generate_world(path: &str) -> Result<(), Error> {
    // floor is sphere that is flattened with matte texture
    let floor = Sphere {
        transform: Matrix::scaling(10.0, 0.01, 10.0),
        material: Material {
            color: Color::new(1.0, 0.9, 0.9),
            diffuse: 0.7,
            specular: 0.0,
            ..Material::default()
        },
        ..Sphere::default()
    };

    // left_wall is sphere that is rotated into place
    let left_wall = Sphere {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-PI / 4.0)
            * Matrix::rotation_x(PI / 2.0)
            * Matrix::scaling(10.0, 0.01, 10.0),
        material: floor.material,
        ..Sphere::default()
    };

    // right_wall is sphere that is rotated into place
    let right_wall = Sphere {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(PI / 4.0)
            * Matrix::rotation_x(PI / 2.0)
            * Matrix::scaling(10.0, 0.01, 10.0),
        material: floor.material,
        ..Sphere::default()
    };

    let middle_sphere = Sphere {
        transform: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let right_sphere = Sphere {
        transform: Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let left_sphere = Sphere {
        transform: Matrix::translation(-1.0, 1.0, -0.75)
            * Matrix::scaling(0.33, 0.33, 0.33)
            * Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        material: Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let mut world = World::new();
    world.objects.push(WorldShape::Sphere(floor));
    world.objects.push(WorldShape::Sphere(left_wall));
    world.objects.push(WorldShape::Sphere(right_wall));
    world.objects.push(WorldShape::Sphere(middle_sphere));
    world.objects.push(WorldShape::Sphere(right_sphere));
    world.objects.push(WorldShape::Sphere(left_sphere));

    world.light = Light::new(
        Coordinates::new_point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );

    // let mut camera = Camera::new(250, 125, PI / 3.0);
    let mut camera = Camera::new(764, 528, PI / 3.0);
    camera.transform = Matrix::view_transform(
        Coordinates::new_point(0.0, 1.5, -5.0),
        Coordinates::new_point(0.0, 1.0, 0.0),
        Coordinates::new_vector(0.0, 1.0, 0.0),
    );

    write(path, camera.render(&world).to_ppm_format())?;

    Ok(())
}
