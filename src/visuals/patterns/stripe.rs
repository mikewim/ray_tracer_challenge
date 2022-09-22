use crate::base_types::Point;
use crate::visuals::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn stripe_at(self, point: Point) -> Color {
        if (point.0.floor() % 2.0) == 0.0 {
            return self.a;
        }

        self.b
    }
}

#[cfg(test)]
mod test {
    use super::StripePattern;
    use crate::base_types::Coordinates;
    use crate::visuals::{BLACK, WHITE};

    #[test]
    fn constant_in_y() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 0.0, 0.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 1.0, 0.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 2.0, 0.0)),
            WHITE
        );
    }

    #[test]
    fn constant_in_z() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 0.0, 0.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 0.0, 1.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 0.0, 2.0)),
            WHITE
        );
    }

    #[test]
    fn alternate_in_x() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.0, 0.0, 0.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(0.9, 0.0, 0.0)),
            WHITE
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(1.0, 0.0, 0.0)),
            BLACK
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(-0.1, 0.0, 0.0)),
            BLACK
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(-1.0, 0.0, 0.0)),
            BLACK
        );

        assert_eq!(
            stripe_pattern.stripe_at(Coordinates::new_point(-1.1, 0.0, 0.0)),
            WHITE
        );
    }
}
