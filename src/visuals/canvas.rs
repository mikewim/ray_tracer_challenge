use std::io::{Error, ErrorKind};

use image::ImageBuffer;
use webp::{Encoder, WebPMemory};

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

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // do bounds check
        if (self.height - y) > self.height || (self.width - x) > self.width {
            return;
        }

        self.grid[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        // self.grid[self.height - y][x]
        self.grid[y][x]
    }

    // saves canvas as webp file
    pub fn save_canvas(&self, path: &str) -> Result<(), Error> {
        let mut image_buff = ImageBuffer::new(self.width as u32 + 1, self.height as u32 + 1);

        for (y, _) in self.grid.iter().enumerate() {
            for (x, color) in self.grid[y].iter().enumerate() {
                image_buff.put_pixel(x as u32, y as u32, image::Rgb(color.to_vec()));
            }
        }

        let encoder: Encoder =
            Encoder::from_rgb(&image_buff, self.width as u32 + 1, self.height as u32 + 1);
        let webp: WebPMemory = encoder.encode(65f32);

        match std::fs::write(path, webp.as_ref()) {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::new(ErrorKind::InvalidInput, error.to_string())),
        }
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

        for y in 0..height {
            for x in 0..width {
                assert_eq!(canvas.pixel_at(x, y), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 22);
        canvas.write_pixel(4, 8, Color::new(1.0, 27.0, 44.0));
        canvas.write_pixel(9, 21, Color::new(8.0, 42.5, 200.0));
        canvas.write_pixel(0, 0, Color::new(24.0, 42.5, 244.0));

        assert_eq!(canvas.pixel_at(4, 8), Color::new(1.0, 27.0, 44.0));
        assert_eq!(canvas.pixel_at(9, 21), Color::new(8.0, 42.5, 200.0));
        assert_eq!(canvas.pixel_at(0, 0), Color::new(24.0, 42.5, 244.0));
    }
}
