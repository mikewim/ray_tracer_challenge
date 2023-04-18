use std::io::Error;

use crate::base_types::{Point, Ray, Vector};
use crate::matrices::Matrix;
use crate::visuals::{Canvas, Color};
use crate::world::{Object, Sphere};

pub fn generate_sphere(dir: &str) -> Result<(), Error> {
    let mut cnvs = Canvas::new(500, 500);

    let mut sphere = Sphere::default();
    sphere.set_transform(
        Matrix::scaling(2.0, 2.0, 2.0).mul(&Matrix::translation(250.0, 250.0, 10.0)),
    );

    for i in 0..500 {
        for j in 0..500 {
            let ray = Ray::new(
                Point::new_point(i as f64, j as f64, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
            );
            if sphere.intersect(ray).is_some() {
                cnvs.write_pixel(i, j, Color(1.0, 0.5, 0.5));
            }
        }
    }

    cnvs.save_canvas(format!("{}/{}", dir, "sphere.webp").as_str())
}
