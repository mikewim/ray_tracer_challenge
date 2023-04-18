pub struct Matrix2D {
    rows: [[f64; 2]; 2],
}

impl Matrix2D {
    pub fn new_with_rows(rows: [[f64; 2]; 2]) -> Self {
        Self { rows }
    }

    pub fn determinant(&self) -> f64 {
        self.rows[0][0] * self.rows[1][1] - self.rows[1][0] * self.rows[0][1]
    }
}

#[cfg(test)]
mod test {
    use super::Matrix2D;

    #[test]
    fn determinant() {
        let row1 = [1.0, 5.0];
        let row2 = [-3.0, 2.0];

        let mut rows = [[0.0; 2]; 2];
        rows[0] = row1;
        rows[1] = row2;

        let matrix = Matrix2D::new_with_rows(rows);

        assert_eq!(matrix.determinant(), 17.0);
    }
}
