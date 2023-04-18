use super::{Color, Pattern};
use crate::base_types::Point;
use crate::matrices::Matrix;

pub struct GradientPattern {
    color_a: Color,
    color_b: Color,
    transform: Matrix,
    nested_patterns: Option<[Box<dyn Pattern>; 2]>,
}

impl GradientPattern {
    pub fn new(
        color_a: Color,
        color_b: Color,
        nested_patterns: Option<[Box<dyn Pattern>; 2]>,
    ) -> Self {
        Self {
            color_a,
            color_b,
            transform: Matrix::new_identity(),
            nested_patterns,
        }
    }
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: Point) -> Color {
        let x_fraction = point.x - point.x.floor();

        if let Some(nested_patterns) = self.nested_patterns.as_ref() {
            let color_a = nested_patterns[0].color_at(point);
            let color_b = nested_patterns[1].color_at(point);
            let color_diff = color_b - self.color_a;

            return color_a + color_diff * x_fraction;
        }

        let color_diff = self.color_b - self.color_a;
        self.color_a + color_diff * x_fraction
    }

    fn clone_pattern(&self) -> Box<dyn Pattern> {
        let mut nested_patterns = None;
        if let Some(clone_patterns) = self.nested_patterns.as_ref() {
            nested_patterns = Some([
                clone_patterns[0].clone_pattern(),
                clone_patterns[1].clone_pattern(),
            ]);
        }

        Box::new(Self {
            color_a: self.color_a,
            color_b: self.color_b,
            transform: self.transform.clone(),
            nested_patterns,
        })
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_at_works() {
        let color_a = Color::new(1.0, 1.0, 1.0);
        let color_b = Color::new(0.0, 0.0, 0.0);
        let pattern = GradientPattern::new(color_a, color_b, None);

        assert_eq!(pattern.color_at(Point::new_point(0.0, 0.0, 0.0)), color_a);
        assert_eq!(
            pattern.color_at(Point::new_point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(Point::new_point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(Point::new_point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
