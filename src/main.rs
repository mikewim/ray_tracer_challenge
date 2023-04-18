use std::io::Error;

mod base_types;
mod matrices;
mod utils;
mod visuals;
mod world;

#[allow(dead_code)]
mod examples;

const OUTPUT_DIR: &str = "./examples";

fn main() -> Result<(), Error> {
    // examples::generate_projectile(OUTPUT_DIR)?;
    // examples::generate_clock(OUTPUT_DIR)?;
    // examples::generate_sphere(OUTPUT_DIR)?;
    // examples::generate_shaded_sphere(OUTPUT_DIR)?;
    // examples::generate_perturbed(OUTPUT_DIR)?;
    // examples::generate_world(OUTPUT_DIR)?;
    examples::generate_plane(OUTPUT_DIR)?;

    Ok(())
}
