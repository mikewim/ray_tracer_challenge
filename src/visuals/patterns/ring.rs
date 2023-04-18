use super::{Color, Pattern};
use crate::base_types::Point;
use crate::matrices::Matrix;

pub struct RingPattern {
    color_a: Color,
    color_b: Color,
    transform: Matrix,
    nested_patterns: Option<[Box<dyn Pattern>; 2]>,
}

impl RingPattern {
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

impl Pattern for RingPattern {
    fn color_at(&self, point: Point) -> Color {
        if ((point.x.powi(2) + point.z.powi(2)).sqrt().floor() as isize) % 2 == 0 {
            if self.nested_patterns.is_some() {
                return self.nested_patterns.as_ref().unwrap()[0].color_at(point);
            }

            return self.color_a;
        }

        if self.nested_patterns.is_some() {
            return self.nested_patterns.as_ref().unwrap()[1].color_at(point);
        }

        self.color_b
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
        let color_a = Color::new(0.0, 0.0, 0.0);
        let color_b = Color::new(1.0, 1.0, 1.0);
        let pattern = RingPattern::new(color_a, color_b, None);

        assert_eq!(pattern.color_at(Point::new_point(0.0, 0.0, 0.0)), color_a);
        assert_eq!(pattern.color_at(Point::new_point(1.0, 0.0, 0.0)), color_b);
        assert_eq!(pattern.color_at(Point::new_point(0.0, 0.0, 1.0)), color_b);
        assert_eq!(
            pattern.color_at(Point::new_point(0.708, 0.0, 0.708)),
            color_b
        );
    }
}
