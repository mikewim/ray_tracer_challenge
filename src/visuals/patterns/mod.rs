use super::Color;
use crate::base_types::Point;
use crate::matrices::Matrix;

mod checker;
mod gradient;
mod perturb;
mod ring;
mod striped;

pub use checker::*;
pub use gradient::*;
pub use perturb::*;
pub use ring::*;
pub use striped::*;

pub trait Pattern {
    fn color_at(&self, point: Point) -> Color;
    fn clone_pattern(&self) -> Box<dyn Pattern>;
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: Matrix);
}
