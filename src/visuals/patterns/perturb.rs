use super::{Color, Pattern};
use crate::base_types::Point;
use crate::matrices::Matrix;

pub struct PerturbPattern {
    pattern: Box<dyn Pattern>,
}

impl PerturbPattern {
    pub fn new(pattern: Box<dyn Pattern>) -> Self {
        Self { pattern }
    }
}

impl Pattern for PerturbPattern {
    // TODO: finish this
    fn color_at(&self, point: Point) -> Color {
        // simlex noise: https://en.wikipedia.org/wiki/Simplex_noise
        // one to consider --> pink noise: https://en.wikipedia.org/wiki/Pink_noise
        let num_of_dimensions = 3.0;
        let noise_base = (point.x + point.y + point.z)
            * (((num_of_dimensions + 1.0_f64).sqrt() - 1.0) / num_of_dimensions);
        let perturbed_point = point + noise_base;
        self.pattern.color_at(perturbed_point)
    }

    fn clone_pattern(&self) -> Box<dyn Pattern> {
        Box::new(Self {
            pattern: self.pattern.clone_pattern(),
        })
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.pattern.set_transform(transform);
    }

    fn get_transform(&self) -> Matrix {
        self.pattern.get_transform()
    }
}
