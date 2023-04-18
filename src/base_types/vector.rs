use super::Coordinates;

pub type Vector = Coordinates;

impl Vector {
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, vec: Self) -> f64 {
        self.x.mul_add(vec.x, self.y.mul_add(vec.y, self.z * vec.z))
    }

    pub fn cross(self, vec: Self) -> Self {
        Self {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
            w: 0.0,
        }
    }

    pub fn reflect(self, vec: Self) -> Self {
        self - (vec * 2.0 * self.dot(vec))
    }
}

#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn add() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);
        let vec2 = Vector::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1 + vec2,
            Vector::new_vector(1.01 - 1.01, 3.0 + 13.254, 5.5 - 42.0)
        );
    }

    #[test]
    fn sub() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);
        let vec2 = Vector::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1 - vec2,
            Vector::new_vector(1.01 + 1.01, 3.0 - 13.254, 5.5 + 42.0)
        );
    }

    #[test]
    fn mul_scalar() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);

        assert_eq!(
            vec1 * 2.0,
            Vector::new_vector(1.01 * 2.0, 3.0 * 2.0, 5.5 * 2.0)
        );
    }

    #[test]
    fn div_scalar() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);

        assert_eq!(
            vec1 / 2.0,
            Vector::new_vector(1.01 / 2.0, 3.0 / 2.0, 5.5 / 2.0)
        );
    }

    #[test]
    fn dot() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);
        let vec2 = Vector::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(vec1.dot(vec2), 1.01 * -1.01 + 3.0 * 13.254 + 5.5 * -42.0);
    }

    #[test]
    fn cross() {
        let vec1 = Vector::new_vector(1.01, 3.0, 5.5);
        let vec2 = Vector::new_vector(-1.01, 13.254, -42.0);

        assert_eq!(
            vec1.cross(vec2),
            Vector::new_vector(
                3.0 * -42.0 - 13.254 * 5.5,
                5.5 * -1.01 - -42.0 * 1.01,
                1.01 * 13.254 - -1.01 * 3.0,
            )
        );
    }

    #[test]
    fn reflect() {
        let vec1 = Vector::new_vector(1.0, -1.0, 0.0);
        let vec2 = Vector::new_vector(0.0, 1.0, 0.0);

        assert!(vec1.reflect(vec2).equal(Vector::new_vector(1.0, 1.0, 0.0)));
    }

    #[test]
    fn reflect_slanted() {
        let vec1 = Vector::new_vector(0.0, -1.0, 0.0);
        let vec2 = Vector::new_vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        assert!(vec1.reflect(vec2).equal(Vector::new_vector(1.0, 0.0, 0.0)));
    }
}
