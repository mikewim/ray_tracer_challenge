pub mod formats;

use crate::visuals::Color;

pub struct Canvas {
    grid: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let rows = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];

        // -1 to account for zero based indexing
        Self {
            grid: rows,
            width: width - 1,
            height: height - 1,
        }
    }

    pub fn write_pixel(&mut self, mut x: usize, mut y: usize, color: Color) {
        while y > self.height {
            y -= self.height
        }

        while x > self.width {
            x -= self.width
        }
        self.grid[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.grid[x][y]
    }

    pub fn to_ppm_format(&self) -> String {
        formats::canvas_to_ppm(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let width = 10;
        let height = 22;
        let canvas = Canvas::new(width, height);

        assert_eq!(canvas.grid.len(), height);
        assert_eq!(canvas.grid[0].len(), width);

        for i in 0..height {
            for j in 0..width {
                assert_eq!(canvas.grid[i][j], Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 22);
        canvas.write_pixel(4, 8, Color::new(1.0, 27.0, 44.0));
        canvas.write_pixel(9, 21, Color::new(8.0, 42.5, 200.0));
        canvas.write_pixel(0, 0, Color::new(24.0, 42.5, 244.0));

        assert_eq!(canvas.grid[8][4], Color::new(1.0, 27.0, 44.0));
        assert_eq!(canvas.grid[21][9], Color::new(8.0, 42.5, 200.0));
        assert_eq!(canvas.grid[0][0], Color::new(24.0, 42.5, 244.0));
    }
}
