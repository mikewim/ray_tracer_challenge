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
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);

        let row1 = [left.x, left.y, left.z, 0.0];
        let row2 = [true_up.x, true_up.y, true_up.z, 0.0];
        let row3 = [-forward.x, -forward.y, -forward.z, 0.0];
        let row4 = [0.0, 0.0, 0.0, 1.0];

        Matrix::new_with_rows([row1, row2, row3, row4])
            .mul(&Matrix::translation(-from.x, -from.y, -from.z))
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    use crate::base_types::{Point, Vector};
    use std::f64::consts::PI;

    #[test]
    fn translate() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0);
        let point = Point::new_point(-3.0, 4.0, 5.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn translate_inverse() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0).inverse();
        let point = Point::new_point(-3.0, 4.0, 5.0);

        assert_eq!(
            matrix.unwrap().coords_mul(point),
            Point::new_point(-8.0, 7.0, 3.0)
        );
    }

    // tranlsation does not affect vector components
    #[test]
    fn translate_vector() {
        let matrix = Matrix::translation(5.0, -3.0, 2.0);
        let vector = Vector::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(matrix.coords_mul(vector), vector);
    }

    #[test]
    fn scaling() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let point = Point::new_point(-4.0, 6.0, 8.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_inverse() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0).inverse();
        let point = Point::new_point(-4.0, 6.0, 8.0);

        assert_eq!(
            matrix.unwrap().coords_mul(point),
            Point::new_point(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn scale_vector() {
        let matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let vec = Vector::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(matrix.coords_mul(vec), Vector::new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflection() {
        let matrix = Matrix::scaling(-1.0, 1.0, 1.0);
        let point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_x() {
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let point = Point::new_point(0.0, 1.0, 0.0);

        assert!((half_quarter.coords_mul(point)).equal(Point::new_point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            2.0_f64.sqrt() / 2.0
        )));

        assert!((full_quarter.coords_mul(point)).equal(Point::new_point(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotation_x_inverse() {
        let half_quarter = Matrix::rotation_x(PI / 4.0).inverse();
        let point = Point::new_point(0.0, 1.0, 0.0);

        assert!(
            (half_quarter.unwrap().coords_mul(point)).equal(Point::new_point(
                0.0,
                2.0_f64.sqrt() / 2.0,
                -2.0_f64.sqrt() / 2.0
            ))
        );
    }

    #[test]
    fn rotation_y() {
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        let point = Point::new_point(0.0, 0.0, 1.0);

        assert!((half_quarter.coords_mul(point)).equal(Point::new_point(
            2.0_f64.sqrt() / 2.0,
            0.0,
            2.0_f64.sqrt() / 2.0
        )));

        assert!((full_quarter.coords_mul(point)).equal(Point::new_point(1.0, 0.0, 0.0)));
    }

    #[test]
    fn rotation_z() {
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        let point = Point::new_point(0.0, 1.0, 0.0);

        assert!((half_quarter.coords_mul(point)).equal(Point::new_point(
            -2.0_f64.sqrt() / 2.0,
            2.0_f64.sqrt() / 2.0,
            0.0
        )));

        assert!((full_quarter.coords_mul(point)).equal(Point::new_point(-1.0, 0.0, 0.0)));
    }

    #[test]
    fn shearing() {
        let mut matrix = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(5.0, 3.0, 4.0));

        matrix = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(6.0, 3.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(2.0, 5.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(2.0, 7.0, 4.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(2.0, 3.0, 6.0));

        matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        point = Point::new_point(2.0, 3.0, 4.0);

        assert_eq!(matrix.coords_mul(point), Point::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn chaining() {
        let rotation = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let point = Point::new_point(1.0, 0.0, 1.0);

        let transformed_point = translation.mul(&scaling).mul(&rotation).coords_mul(point);
        assert!(transformed_point.equal(Point::new_point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn chaining_sequence() {
        let rotation = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let point = Point::new_point(1.0, 0.0, 1.0);

        let mut transformed_point = rotation.coords_mul(point);
        assert!(transformed_point.equal(Point::new_point(1.0, -1.0, 0.0)));

        transformed_point = scaling.coords_mul(transformed_point);
        assert!(transformed_point.equal(Point::new_point(5.0, -5.0, 0.0)));

        transformed_point = translation.coords_mul(transformed_point);
        assert!(transformed_point.equal(Point::new_point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn view_transform_default() {
        let from = Point::new_point(0.0, 0.0, 0.0);
        let to = Point::new_point(0.0, 0.0, -1.0);
        let up = Vector::new_vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::new_identity());
    }

    #[test]
    fn view_transform_positive_z() {
        let from = Point::new_point(0.0, 0.0, 0.0);
        let to = Point::new_point(0.0, 0.0, 1.0);
        let up = Vector::new_vector(0.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transform_move_world() {
        let from = Point::new_point(0.0, 0.0, 8.0);
        let to = Point::new_point(0.0, 0.0, 0.0);
        let up = Vector::new_vector(0.0, 1.0, 0.0);

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

        let from = Point::new_point(1.0, 3.0, 2.0);
        let to = Point::new_point(4.0, -2.0, 8.0);
        let up = Vector::new_vector(1.0, 1.0, 0.0);

        let t = Matrix::view_transform(from, to, up);
        assert!(t.equal(&Matrix::new_with_rows(rows)));
    }
}
