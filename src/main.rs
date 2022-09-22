mod base_types;
mod canvas;
mod examples;
mod matrices;
mod shapes;
mod utils;
mod visuals;
mod world;

pub use examples::*;

use std::io::Error;

fn main() -> Result<(), Error> {
    let output_dir = "examples";
    // generate_projectile("projectile.ppm")?;
    // generate_clock("clock.ppm")?;
    // generate_sphere("sphere.ppm")?;
    // generate_3d_sphere("sphere_3d.ppm")?;
    // generate_world("world.ppm")?;
    generate_world_with_plane(format!("{}/{}", output_dir, "plane.ppm").as_str())?;

    Ok(())
}
