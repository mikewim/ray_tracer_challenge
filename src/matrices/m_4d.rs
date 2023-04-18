use super::m_3d::Matrix3D;
use crate::base_types::Coordinates;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct Matrix {
    pub rows: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new() -> Self {
        let rows = [[0.0; 4]; 4];

        Self { rows }
    }

    pub fn new_with_rows(rows: [[f64; 4]; 4]) -> Self {
        Self { rows }
    }

    pub fn new_identity() -> Self {
        let mut rows = [[0.0; 4]; 4];

        rows[0][0] = 1.0;
        rows[1][1] = 1.0;
        rows[2][2] = 1.0;
        rows[3][3] = 1.0;

        Self { rows }
    }

    pub fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();
        // no determinant means this matrix cannot be inverted
        if determinant == 0.0 {
            return None;
        }

        let mut rows = [[0.0; 4]; 4];

        for (i, row) in rows.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = self.cofactor(i, j);
            }
        }

        let inv_matrix = Self::new_with_rows(rows).transpose();
        Some(inv_matrix.scalar_div(determinant))
    }

    pub fn transpose(&self) -> Self {
        let mut rows = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in i..4 {
                let temp = self.rows[i][j];
                rows[i][j] = self.rows[j][i];
                rows[j][i] = temp;
            }
        }

        Self::new_with_rows(rows)
    }

    pub fn mul(&self, _rhs: &Self) -> Self {
        let mut new_rows = [[0.0; 4]; 4];
        for (i, row) in new_rows.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = self.rows[i][0] * _rhs.rows[0][j]
                    + self.rows[i][1] * _rhs.rows[1][j]
                    + self.rows[i][2] * _rhs.rows[2][j]
                    + self.rows[i][3] * _rhs.rows[3][j];
            }
        }

        Self { rows: new_rows }
    }

    pub fn coords_mul(&self, _rhs: Coordinates) -> Coordinates {
        let mut new_coords = Coordinates {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: _rhs.w,
        };

        new_coords.x = self.rows[0][0] * _rhs.x
            + self.rows[0][1] * _rhs.y
            + self.rows[0][2] * _rhs.z
            + self.rows[0][3] * _rhs.w;

        new_coords.y = self.rows[1][0] * _rhs.x
            + self.rows[1][1] * _rhs.y
            + self.rows[1][2] * _rhs.z
            + self.rows[1][3] * _rhs.w;

        new_coords.z = self.rows[2][0] * _rhs.x
            + self.rows[2][1] * _rhs.y
            + self.rows[2][2] * _rhs.z
            + self.rows[2][3] * _rhs.w;

        new_coords
    }

    fn scalar_div(self, _rhs: f64) -> Self {
        let mut new_rows = [[0.0; 4]; 4];
        for (i, row) in new_rows.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = self.rows[i][j] / _rhs;
            }
        }

        Self { rows: new_rows }
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 || minor == 0.0 {
            return minor;
        }

        -minor
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.sub_matrix(row, col).determinant()
    }

    fn determinant(&self) -> f64 {
        self.rows[0][0] * self.cofactor(0, 0)
            + self.rows[0][1] * self.cofactor(0, 1)
            + self.rows[0][2] * self.cofactor(0, 2)
            + self.rows[0][3] * self.cofactor(0, 3)
    }

    fn sub_matrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Matrix3D {
        let mut rows = [[0.0; 3]; 3];

        let mut parent_row = 0;
        for row in rows.iter_mut() {
            if parent_row == row_to_exclude {
                parent_row += 1;
            }

            let mut parent_col = 0;
            for col in row.iter_mut() {
                if parent_col == col_to_exclude {
                    parent_col += 1;
                }

                *col = self.rows[parent_row][parent_col];
                parent_col += 1;
            }

            parent_row += 1;
        }

        Matrix3D::new_with_rows(rows)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl Matrix {
    pub fn equal(&self, _rhs: &Self) -> bool {
        let col_length = self.rows.len();
        let row_length = self.rows[0].len();
        for i in 0..row_length {
            for j in 0..col_length {
                if !crate::utils::float_equal(self.rows[i][j], _rhs.rows[i][j]) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base_types::Vector;

    #[test]
    fn equal() {
        let matrix1 = Matrix::new();
        let matrix2 = Matrix::new();

        // zero matrices are equal
        assert_eq!(matrix1, matrix2);

        let row1 = [1.0, 2.0, 3.0, 4.0];
        let row2 = [4.0, 3.0, 2.0, 1.0];
        let row3 = [7.0, 8.0, 9.0, 0.01];
        let row4 = [-12.0, 0.25, 1.5, 5.0];

        // matrices with identical rows are equal
        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let matrix3 = Matrix::new_with_rows(rows);
        let matrix4 = Matrix::new_with_rows(rows);

        assert_eq!(matrix3, matrix4);

        // matrices with not identical rows are not equal
        assert!(matrix3 != matrix1);
    }

    #[test]
    fn mul() {
        let row1 = [1.0, 2.0, 3.0, 4.0];
        let row2 = [5.0, 6.0, 7.0, 8.0];
        let row3 = [9.0, 8.0, 7.0, 6.0];
        let row4 = [5.0, 4.0, 3.0, 2.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let row5 = [-2.0, 1.0, 2.0, 3.0];
        let row6 = [3.0, 2.0, 1.0, -1.0];
        let row7 = [4.0, 3.0, 6.0, 5.0];
        let row8 = [1.0, 2.0, 7.0, 8.0];

        let mut rows2 = [[0.0; 4]; 4];
        rows2[0] = row5;
        rows2[1] = row6;
        rows2[2] = row7;
        rows2[3] = row8;

        let matrix1 = Matrix::new_with_rows(rows);
        let matrix2 = Matrix::new_with_rows(rows2);

        let prod_row1 = [20.0, 22.0, 50.0, 48.0];
        let prod_row2 = [44.0, 54.0, 114.0, 108.0];
        let prod_row3 = [40.0, 58.0, 110.0, 102.0];
        let prod_row4 = [16.0, 26.0, 46.0, 42.0];

        let mut prod_rows = [[0.0; 4]; 4];
        prod_rows[0] = prod_row1;
        prod_rows[1] = prod_row2;
        prod_rows[2] = prod_row3;
        prod_rows[3] = prod_row4;

        assert_eq!(Matrix::new_with_rows(prod_rows), matrix1.mul(&matrix2));
    }

    #[test]
    fn mul_vec() {
        let row1 = [1.0, 2.0, 3.0, 4.0];
        let row2 = [5.0, 6.0, 7.0, 8.0];
        let row3 = [9.0, 8.0, 7.0, 6.0];
        let row4 = [5.0, 4.0, 3.0, 2.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let vec = Vector::new_vector(2.0, -1.0, 4.0);

        assert_eq!(
            Matrix::new_with_rows(rows).coords_mul(vec),
            Vector::new_vector(12.0, 32.0, 38.0)
        );
    }

    #[test]
    fn transpose() {
        let row1 = [0.0, 9.0, 3.0, 0.0];
        let row2 = [9.0, 8.0, 0.0, 8.0];
        let row3 = [1.0, 8.0, 5.0, 3.0];
        let row4 = [0.0, 0.0, 5.0, 8.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let trans_row1 = [0.0, 9.0, 1.0, 0.0];
        let trans_row2 = [9.0, 8.0, 8.0, 0.0];
        let trans_row3 = [3.0, 0.0, 5.0, 5.0];
        let trans_row4 = [0.0, 8.0, 3.0, 8.0];

        let mut trans_rows = [[0.0; 4]; 4];
        trans_rows[0] = trans_row1;
        trans_rows[1] = trans_row2;
        trans_rows[2] = trans_row3;
        trans_rows[3] = trans_row4;

        let matrix = Matrix::new_with_rows(rows);
        let trans_matrix = Matrix::new_with_rows(trans_rows);

        assert_eq!(matrix.transpose(), trans_matrix);
    }

    #[test]
    fn determinant() {
        let row1 = [-2.0, -8.0, 3.0, 5.0];
        let row2 = [-3.0, 1.0, 7.0, 3.0];
        let row3 = [1.0, 2.0, -9.0, 6.0];
        let row4 = [-6.0, 7.0, 7.0, -9.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let matrix = Matrix::new_with_rows(rows);
        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    fn inverse() {
        let row1 = [1.0, 2.0, 3.0, 4.0];
        let row2 = [5.0, 6.0, 7.0, 8.0];
        let row3 = [9.0, 8.0, 7.0, 6.0];
        let row4 = [5.0, 4.0, 3.0, 2.0];

        let mut rows = [[0.0; 4]; 4];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;
        rows[3] = row4;

        let row5 = [-2.0, 1.0, 2.0, 3.0];
        let row6 = [3.0, 2.0, 1.0, -1.0];
        let row7 = [4.0, 3.0, 6.0, 5.0];
        let row8 = [1.0, 2.0, 7.0, 8.0];

        let mut rows2 = [[0.0; 4]; 4];
        rows2[0] = row5;
        rows2[1] = row6;
        rows2[2] = row7;
        rows2[3] = row8;

        let matrix1 = Matrix::new_with_rows(rows);
        let matrix2 = Matrix::new_with_rows(rows2);
        let product_matrix = matrix1.mul(&matrix2);
        let matrix2_inv = matrix2.inverse();

        assert!(matrix2_inv.is_some());
        assert!(matrix1.equal(&(product_matrix.mul(&matrix2_inv.unwrap()))));
    }
}
