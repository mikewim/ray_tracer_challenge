use std::io::Error;

use crate::base_types::{Point, Ray, Vector};
use crate::matrices::Matrix;
use crate::visuals::{Canvas, Color};
use crate::world::{Light, Object, Sphere};

pub fn generate_shaded_sphere(dir: &str) -> Result<(), Error> {
    let mut cnvs = Canvas::new(500, 500);

    let mut sphere = Sphere::default();
    sphere.set_transform(
        Matrix::translation(250.0, 250.0, 10.0).mul(&Matrix::scaling(150.0, 150.0, 150.0)),
    );
    sphere.get_material_mut().color = Color::new(1.0, 0.2, 1.0);
    let light_source = Light::new(
        Point::new_point(100.0, 450.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );

    for i in 0..500 {
        for j in 0..500 {
            let ray = Ray::new(
                Point::new_point(i as f64, j as f64, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
            );
            if let Some(interection) = sphere.intersect(ray) {
                cnvs.write_pixel(
                    i,
                    j,
                    light_source.lighting(
                        &sphere,
                        ray.position(interection[0].distance),
                        -ray.direction,
                        sphere.normal_at(Point::new_point(i as f64, j as f64, 0.0)),
                        false,
                    ),
                );
            }
        }
    }

    cnvs.save_canvas(format!("{}/{}", dir, "shaded_sphere.webp").as_str())
}
