use super::m_2d::Matrix2D;

pub struct Matrix3D {
    rows: [[f64; 3]; 3],
}

impl Matrix3D {
    pub fn new_with_rows(rows: [[f64; 3]; 3]) -> Self {
        Self { rows }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 || minor == 0.0 {
            return minor;
        }

        -minor
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.sub_matrix(row, col).determinant()
    }

    pub fn determinant(&self) -> f64 {
        self.rows[0][0] * self.cofactor(0, 0)
            + self.rows[0][1] * self.cofactor(0, 1)
            + self.rows[0][2] * self.cofactor(0, 2)
    }

    pub fn sub_matrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Matrix2D {
        let mut rows = [[0.0; 2]; 2];

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

        Matrix2D::new_with_rows(rows)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minor() {
        let row1 = [3.0, 5.0, 0.0];
        let row2 = [2.0, -1.0, -7.0];
        let row3 = [6.0, -1.0, 5.0];

        let mut rows = [[0.0; 3]; 3];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;

        let matrix = Matrix3D::new_with_rows(rows);

        assert_eq!(matrix.minor(1, 0), 25.0);
    }

    #[test]
    fn determinant() {
        let row1 = [1.0, 2.0, 6.0];
        let row2 = [-5.0, 8.0, -4.0];
        let row3 = [2.0, 6.0, 4.0];

        let mut rows = [[0.0; 3]; 3];
        rows[0] = row1;
        rows[1] = row2;
        rows[2] = row3;

        let matrix = Matrix3D::new_with_rows(rows);
        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant(), -196.0);
    }
}
