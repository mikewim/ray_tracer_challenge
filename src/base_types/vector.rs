use super::Coordinates;

pub type Vector = Coordinates;

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();

        self / magnitude
    }

    pub fn dot(self, vec: Self) -> f64 {
        self.0.mul_add(vec.0, self.1.mul_add(vec.1, self.2 * vec.2))
    }

    pub fn cross(self, vec: Self) -> Self {
        Self(
            self.1 * vec.2 - self.2 * vec.1,
            self.2 * vec.0 - self.0 * vec.2,
            self.0 * vec.1 - self.1 * vec.0,
            0.0,
        )
    }

    pub fn reflect(self, vec: Self) -> Self {
        self - (vec * 2.0 * self.dot(vec))
    }
}

#[cfg(test)]
mod test {
    use super::Coordinates;

    #[test]
    fn add() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);
        let vec2 = Coordinates::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1 + vec2,
            Coordinates::new_vector(1.01 - 1.01, 3.0 + 13.254, 5.5 - 42.0)
        );
    }

    #[test]
    fn sub() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);
        let vec2 = Coordinates::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1 - vec2,
            Coordinates::new_vector(1.01 + 1.01, 3.0 - 13.254, 5.5 + 42.0)
        );
    }

    #[test]
    fn mul_scalar() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);

        assert_eq!(
            vec1 * 2.0,
            Coordinates::new_vector(1.01 * 2.0, 3.0 * 2.0, 5.5 * 2.0)
        );
    }

    #[test]
    fn div_scalar() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);

        assert_eq!(
            vec1 / 2.0,
            Coordinates::new_vector(1.01 / 2.0, 3.0 / 2.0, 5.5 / 2.0)
        );
    }

    #[test]
    fn dot() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);
        let vec2 = Coordinates::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(vec1.dot(vec2), 1.01 * -1.01 + 3.0 * 13.254 + 5.5 * -42.0);
    }

    #[test]
    fn cross() {
        let vec1 = Coordinates::new_vector(1.01, 3.0, 5.5);
        let vec2 = Coordinates::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1.cross(vec2),
            Coordinates::new_vector(
                3.0 * -42.0 - 13.254 * 5.5,
                5.5 * -1.01 - -42.0 * 1.01,
                1.01 * 13.254 - -1.01 * 3.0,
            )
        );
    }

    #[test]
    fn reflect() {
        let vec1 = Coordinates::new_vector(1.0, -1.0, 0.0);
        let vec2 = Coordinates::new_vector(0.0, 1.0, 0.0);

        assert_eq!(vec1.reflect(vec2), Coordinates::new_vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_slanted() {
        let vec1 = Coordinates::new_vector(0.0, -1.0, 0.0);
        let vec2 =
            Coordinates::new_vector((2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0);

        assert!(vec1
            .reflect(vec2)
            .equal(Coordinates::new_vector(1.0, 0.0, 0.0)));
    }
}
