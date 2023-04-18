use std::f64::consts::PI;
use std::io::Error;

use crate::base_types::{Point, Vector};
use crate::matrices::Matrix;
use crate::visuals::{Canvas, Color};

pub fn generate_clock(dir: &str) -> Result<(), Error> {
    let mut cnvs = Canvas::new(500, 500);

    let origin = Point::new_point(250.0, 250.0, 0.0);
    let original_vec = Vector::new_vector(0.0, 50.0, 0.0);

    for i in 0..12 {
        let rotation = Matrix::rotation_z(PI / 6.0 * (i as f64)).coords_mul(original_vec);
        let new_point = origin + rotation;
        cnvs.write_pixel(
            new_point.x as usize,
            new_point.y as usize,
            Color(1.0, 0.5, 0.5),
        );
    }

    cnvs.save_canvas(format!("{}/{}", dir, "clock.webp").as_str())
}
