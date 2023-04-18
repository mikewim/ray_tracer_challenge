use std::io::Error;

use crate::base_types::{Point, Vector};
use crate::visuals::{Canvas, Color};

struct Projectile {
    point: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

pub fn generate_projectile(dir: &str) -> Result<(), Error> {
    let mut proj = Projectile {
        point: Point::new_point(0.0, 1.0, 0.0),
        velocity: Vector::new_vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = &Environment {
        gravity: Vector::new_vector(0.0, -0.1, 0.0),
        wind: Vector::new_vector(-0.01, 0.0, 0.0),
    };

    let mut cnvs = Canvas::new(900, 500);

    for _ in 0..1000 {
        proj = tick(env, proj);
        cnvs.write_pixel(
            proj.point.x as usize,
            proj.point.y as usize,
            Color(1.0, 0.5, 0.5),
        );
    }

    cnvs.save_canvas(format!("{}/{}", dir, "projectile.webp").as_str())
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        point: proj.point + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
