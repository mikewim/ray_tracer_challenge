use super::World;
use crate::base_types::{Point, Ray};
use crate::matrices::Matrix;
use crate::visuals::Canvas;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    pub field_of_view: f64,
    pub transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        // basically split full triangle view into right triangle
        // by cutting field of view in half. Tangent is equal to
        // the width of half the canvas, since the camera is one unit away
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);

        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / (hsize as f64);

        Self {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size,
            field_of_view,
            transform: Matrix::new_identity(),
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas
        // to the pixel's center
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        // the untransform coordinates of the pixel in world space
        // (camera looks toward -z, so +x is to the left)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let camera_transform_inv = self.transform.inverse().unwrap();
        let origin = camera_transform_inv.coords_mul(Point::new_point(0.0, 0.0, 0.0));

        // canvas is at z = -1.0
        let pixel = camera_transform_inv.coords_mul(Point::new_point(world_x, world_y, -1.0));
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, None);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod test {
    use super::{Camera, World};
    use crate::base_types::{Point, Vector};
    use crate::matrices::Matrix;
    use crate::utils::float_equal;
    use crate::visuals::Color;
    use std::f64::consts::PI;

    #[test]
    fn pixel_size() {
        let horiz_camera = Camera::new(200, 125, PI / 2.0);

        assert!(float_equal(horiz_camera.pixel_size, 0.01));

        let vert_camera = Camera::new(125, 200, PI / 2.0);

        assert!(float_equal(vert_camera.pixel_size, 0.01));
    }

    #[test]
    fn ray_for_pixel_center() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new_point(0.0, 0.0, 0.0));
        assert!(ray.direction.equal(Vector::new_vector(0.0, 0.0, -1.0)));
    }

    #[test]
    fn ray_for_pixel_corner() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.origin, Point::new_point(0.0, 0.0, 0.0));
        assert!(ray
            .direction
            .equal(Vector::new_vector(0.66519, 0.33259, -0.66851)));
    }

    #[test]
    fn ray_for_pixel_with_transform() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.transform = Matrix::rotation_y(PI / 4.0).mul(&Matrix::translation(0.0, -2.0, 5.0));
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new_point(0.0, 2.0, -5.0));
        assert!(ray.direction.equal(Vector::new_vector(
            2.0_f64.sqrt() / 2.0,
            0.0,
            -2.0_f64.sqrt() / 2.0
        )));
    }

    #[test]
    fn render_default() {
        let world = World::default();
        let mut camera = Camera::new(11, 11, PI / 2.0);
        let from = Point::new_point(0.0, 0.0, -5.0);
        let to = Point::new_point(0.0, 0.0, 0.0);
        let up = Point::new_point(0.0, 1.0, 0.0);
        camera.transform = Matrix::view_transform(from, to, up);

        let image = camera.render(&world);
        assert!(image
            .pixel_at(5, 5)
            .equal(Color::new(0.38066, 0.47583, 0.2855)));
    }
}
