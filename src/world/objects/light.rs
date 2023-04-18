use super::Object;
use crate::base_types::{Point, Vector};
use crate::visuals::Color;

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Point,
    pub color: Color,
}

impl Light {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }

    pub fn lighting(
        self: Light,
        object: &dyn Object,
        position: Point,
        eye_normal: Vector,
        surface_normal: Vector,
        is_in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color
        let effective_color = object.color_at(position) * self.color;
        let material = object.get_material();

        // direction to the light source
        let light_direction = (self.position - position).normalize();

        // ambient light contribution
        let ambient = effective_color * material.ambient;

        let diffuse: Color;
        let specular: Color;

        // cosine between light normal and surface normal
        let light_dot_normal = light_direction.dot(surface_normal);

        // if it's negative, light is behind surface
        if is_in_shadow || light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            // diffuse contribution
            diffuse = effective_color * material.diffuse * light_dot_normal;

            // get the cosine of the angle between the reflection vector and the
            // eye vector
            let light_reflection = (-light_direction).reflect(surface_normal);
            let reflection_dot_eye = light_reflection.dot(eye_normal);

            // if it's negative, the light is reflecting away from the eye
            if reflection_dot_eye < 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                // specular contribution
                let factor = reflection_dot_eye.powf(material.shininess);
                specular = self.color * material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::Sphere;

    #[test]
    fn lighting_eye_between() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(Point::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            light_point.lighting(&sphere, position, eye_normal, surface_normal, false),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_eye_45_offset() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(Point::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            light_point.lighting(&sphere, position, eye_normal, surface_normal, false),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lighting_light_45_offset() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Point::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert!(light_point
            .lighting(&sphere, position, eye_normal, surface_normal, false)
            .equal(Color::new(0.7364, 0.7364, 0.7364)));
    }

    #[test]
    fn lighting_light_opposite_offset() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Point::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert!(light_point
            .lighting(&sphere, position, eye_normal, surface_normal, false)
            .equal(Color::new(1.6364, 1.6364, 1.6364)));
    }

    #[test]
    fn lighting_light_behind_surface() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(Point::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            light_point.lighting(&sphere, position, eye_normal, surface_normal, false),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_in_shadow() {
        let sphere = Sphere::default();
        let position = Point::new_point(0.0, 0.0, 0.0);
        let eye_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Vector::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(Point::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            light_point.lighting(&sphere, position, eye_normal, surface_normal, true),
            Color::new(0.1, 0.1, 0.1)
        );
    }
}
