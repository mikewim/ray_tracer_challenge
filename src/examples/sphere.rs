use std::fs::write;
use std::io::Error;

use crate::base_types::{hit, Coordinates, Ray};
use crate::canvas::Canvas;
use crate::matrices::Matrix;
use crate::shapes::{Shape, Sphere};
use crate::visuals::{Color, Material};

pub fn generate_sphere(path: &str) -> Result<(), Error> {
    let canvas_pixels = 100;
    let mut cnvs = Canvas::new(canvas_pixels, canvas_pixels);

    let wall_size = 7;
    let pixel_size = (wall_size as f64) / (canvas_pixels as f64);
    let half = (wall_size as f64) / 2.0;

    let ray_origin = Coordinates::new_point(0.0, 0.0, -5.0);
    let sphere = Sphere {
        center: Coordinates::new_point(ray_origin.0, ray_origin.1, 0.0),
        transform: Matrix::scaling(1.0, 0.5, 1.0),
        material: Material {
            color: Color::new(1.0, 0.0, 0.0),
            ..Material::default()
        },
        ..Sphere::default()
    };
    // sphere.transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    // sphere.transform = Matrix::translation(1.0, 0.0, 0.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            // wall position
            let position = Coordinates::new_point(world_x, world_y, 10.0);
            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).normalize(),
            };

            if let Some(intersections) = sphere.intersect(ray) {
                if hit(intersections).is_some() {
                    cnvs.write_pixel(x, y, sphere.material.color);
                }
            }
        }
    }

    write(path, cnvs.to_ppm_format())?;

    Ok(())
}
