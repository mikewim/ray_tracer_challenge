mod color;
mod patterns;
mod shading;

pub use color::*;
pub use patterns::*;
pub use shading::*;

#[cfg(test)]
pub const WHITE: Color = Color(1.0, 1.0, 1.0);
#[cfg(test)]
pub const BLACK: Color = Color(0.0, 0.0, 0.0);
