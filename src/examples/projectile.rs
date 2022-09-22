use std::fs::write;
use std::io::Error;

use crate::base_types::{Coordinates, Vector};
use crate::canvas::Canvas;
use crate::visuals::Color;

struct Projectile {
    point: Vector,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

pub fn generate_projectile(path: &str) -> Result<(), Error> {
    let mut proj = Projectile {
        point: Coordinates::new_point(0.0, 1.0, 0.0),
        velocity: Coordinates::new_vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = &Environment {
        gravity: Coordinates::new_vector(0.0, -0.1, 0.0),
        wind: Coordinates::new_vector(-0.01, 0.0, 0.0),
    };

    let mut cnvs = Canvas::new(900, 500);

    for _ in 0..1000 {
        proj = tick(env, proj);
        cnvs.write_pixel(
            proj.point.0 as usize,
            proj.point.1 as usize,
            Color(1.0, 0.5, 0.5),
        );
    }

    write(path, cnvs.to_ppm_format())?;

    Ok(())
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        point: proj.point + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
