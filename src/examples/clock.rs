use std::f64::consts::PI;
use std::fs::write;
use std::io::Error;

use crate::base_types::{Coordinates, Point};
use crate::canvas::Canvas;
use crate::matrices::Matrix;
use crate::visuals::Color;

pub fn generate_clock(path: &str) -> Result<(), Error> {
    let mut cnvs = Canvas::new(900, 500);
    let point = Coordinates::new_point(450.0, 250.0, 0.0);

    generate_circle(200.0, point, &mut cnvs)?;
    write(path, cnvs.to_ppm_format())?;

    Ok(())
}

pub fn generate_circle(radius: f64, center: Point, cnvs: &mut Canvas) -> Result<(), Error> {
    let mut theta: f64 = 0.0;

    while theta < 2.0 * PI {
        let translation =
            Matrix::translation(radius * theta.cos(), radius * theta.sin(), 0.0) * center;

        cnvs.write_pixel(
            translation.0 as usize,
            translation.1 as usize,
            Color(1.0, 0.5, 0.5),
        );

        theta += PI / 6.0;
        // theta += PI / 2.0;
    }

    Ok(())
}
