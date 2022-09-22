use super::m_4d::Matrix;
use crate::base_types::{Point, Vector};

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Self::new_identity();
        matrix.rows[0][3] = x;
        matrix.rows[1][3] = y;
        matrix.rows[2][3] = z;

        matrix
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Self::new_identity();
        matrix.rows[0][0] = x;
        matrix.rows[1][1] = y;
        matrix.rows[2][2] = z;

        matrix
    }

    pub fn rotation_x(theta: f64) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let mut matrix = Self::new_identity();
        matrix.rows[1][1] = cos;
        matrix.rows[1][2] = -sin;
        matrix.rows[2][1] = sin;
        matrix.rows[2][2] = cos;

        matrix
    }

    pub fn rotation_y(theta: f64) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let mut matrix = Self::new_identity();
        matrix.rows[0][0] = cos;
        matrix.rows[0][2] = sin;
        matrix.rows[2][0] = -sin;
        matrix.rows[2][2] = cos;

        matrix
    }

    pub fn rotation_z(theta: f64) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let mut matrix = Self::new_identity();
        matrix.rows[0][0] = cos;
        matrix.rows[0][1] = -sin;
        matrix.rows[1][0] = sin;
        matrix.rows[1][1] = cos;

        matrix
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut matrix = Self::new_identity();
        matrix.rows[0][1] = xy;
        matrix.rows[0][2] = xz;
        matrix.rows[1][0] = yx;
        matrix.rows[1][2] = yz;
        matrix.rows[2][0] = zx;
        matrix.rows[2][1] = zy;

        matrix
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Self {
        let forward = (to - from).normalize();
        let up_normal = up.normalize();
        let left = forward.cross(up_normal);
        let true_up = left.cross(forward);

        let row1 = [left.0, left.1, left.2, 0.0];
        let row2 = [true_up.0, true_up.1, true_up.2, 0.0];
        let row3 = [-forward.0, -forward.1, -forward.2, 0.0];
        let row4 = [0.0, 0.0, 0.0, 1.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        Matrix::new_with_rows(rows) * Matrix::translation(-from.0, -from.1, -from.2)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    use crate::base_types::Coordinates;
    use std::f64::consts::PI;

    #[test]
    fn translate() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0);
        let point = Coordinates::new_point(-3.0, 4.0, 5.0);

        assert_eq!(matrix * point, Coordinates::new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn translate_inverse() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0).inverse();
        let point = Coordinates::new_point(-3.0, 4.0, 5.0);

        assert_eq!(
            matrix.unwrap() * point,
            Coordinates::new_point(-8.0, 7.0, 3.0)
        );
    }

    // tranlsation does not affect vector components
    #[test]
    fn translate_vector() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0);
        let vector = Coordinates::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(matrix * vector, vector);
    }

    #[test]
    fn scaling() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let point = Coordinates::new_point(-4.0, 6.0, 8.0);

        assert_eq!(matrix * point, Coordinates::new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_inverse() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0).inverse();
        let point = Coordinates::new_point(-4.0, 6.0, 8.0);

        assert_eq!(
            matrix.unwrap() * point,
            Coordinates::new_point(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn scale_vector() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let vec = Coordinates::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(matrix * vec, Coordinates::new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflection() {
        let matrix = Matrix::scaling(-1.0, 1.0, 1.0);
        let point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_x() {
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let point = Coordinates::new_point(0.0, 1.0, 0.0);

        assert!((half_quarter * point).equal(Coordinates::new_point(
            0.0,
            (2.0 as f64).sqrt() / 2.0,
            (2.0 as f64).sqrt() / 2.0
        )));

        assert!((full_quarter * point).equal(Coordinates::new_point(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotation_x_inverse() {
        let half_quarter = Matrix::rotation_x(PI / 4.0).inverse();
        let point = Coordinates::new_point(0.0, 1.0, 0.0);

        assert!(
            (half_quarter.unwrap() * point).equal(Coordinates::new_point(
                0.0,
                (2.0 as f64).sqrt() / 2.0,
                -(2.0 as f64).sqrt() / 2.0
            ))
        );
    }

    #[test]
    fn rotation_y() {
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        let point = Coordinates::new_point(0.0, 0.0, 1.0);

        assert!((half_quarter * point).equal(Coordinates::new_point(
            (2.0 as f64).sqrt() / 2.0,
            0.0,
            (2.0 as f64).sqrt() / 2.0
        )));

        assert!((full_quarter * point).equal(Coordinates::new_point(1.0, 0.0, 0.0)));
    }

    #[test]
    fn rotation_z() {
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        let point = Coordinates::new_point(0.0, 1.0, 0.0);

        assert!((half_quarter * point).equal(Coordinates::new_point(
            -((2.0 as f64).sqrt() / 2.0),
            (2.0 as f64).sqrt() / 2.0,
            0.0
        )));

        assert!((full_quarter * point).equal(Coordinates::new_point(-1.0, 0.0, 0.0)));
    }

    #[test]
    fn shearing() {
        let mut matrix = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(5.0, 3.0, 4.0));

        matrix = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(6.0, 3.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(2.0, 5.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(2.0, 7.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(2.0, 3.0, 6.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        point = Coordinates::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix * point, Coordinates::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn chaining() {
        let rotation = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let point = Coordinates::new_point(1.0, 0.0, 1.0);

        let transformed_point = translation * scaling * rotation * point;
        assert!(transformed_point.equal(Coordinates::new_point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn chaining_sequence() {
        let rotation = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let point = Coordinates::new_point(1.0, 0.0, 1.0);

        let mut transformed_point = rotation * point;
        assert!(transformed_point.equal(Coordinates::new_point(1.0, -1.0, 0.0)));

        transformed_point = scaling * transformed_point;
        assert!(transformed_point.equal(Coordinates::new_point(5.0, -5.0, 0.0)));

        transformed_point = translation * transformed_point;
        assert!(transformed_point.equal(Coordinates::new_point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn view_transform_default() {
        let from = Coordinates::new_point(0.0, 0.0, 0.0);
        let to = Coordinates::new_point(0.0, 0.0, -1.0);
        let up = Coordinates::new_vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::new_identity());
    }

    #[test]
    fn view_transform_positive_z() {
        let from = Coordinates::new_point(0.0, 0.0, 0.0);
        let to = Coordinates::new_point(0.0, 0.0, 1.0);
        let up = Coordinates::new_vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transform_move_world() {
        let from = Coordinates::new_point(0.0, 0.0, 8.0);
        let to = Coordinates::new_point(0.0, 0.0, 0.0);
        let up = Coordinates::new_vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn view_transform_arbitrary() {
        let row1 = [-0.50709, 0.50709, 0.67612, -2.36643];
        let row2 = [0.76771, 0.60609, 0.12121, -2.82842];
        let row3 = [-0.35856, 0.59761, -0.71713, 0.0];
        let row4 = [0.0, 0.0, 0.0, 1.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let from = Coordinates::new_point(1.0, 3.0, 2.0);
        let to = Coordinates::new_point(4.0, -2.0, 8.0);
        let up = Coordinates::new_vector(1.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert!(t.equal(&Matrix::new_with_rows(rows)));
    }
}
