use super::{Color, StripePattern};
use crate::base_types::{Point, Vector};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<StripePattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<StripePattern>,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        // seemingly randy vals for default from the book
        Self::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, None)
    }
}

pub fn lighting(
    material: Material,
    point_light: Light,
    position: Point,
    eye_normal: Vector,
    surface_normal: Vector,
    is_in_shadow: bool,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * point_light.intensity;

    // direction to the light source
    let light_direction = (point_light.position - position).normalize();

    // ambient light contribution
    let ambient = effective_color * material.ambient;

    let diffuse: Color;
    let specular: Color;

    // cosine between light normal and surface normal
    let light_dot_normal = light_direction.dot(surface_normal);

    // if it's negative, light is behind surface
    if light_dot_normal < 0.0 || is_in_shadow {
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
            specular = point_light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base_types::Coordinates;

    #[test]
    fn lighting_eye_between() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            lighting(
                material,
                light_point,
                position,
                eye_normal,
                surface_normal,
                false
            ),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_eye_45_offset() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal =
            Coordinates::new_vector(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            lighting(
                material,
                light_point,
                position,
                eye_normal,
                surface_normal,
                false
            ),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lighting_light_45_offset() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert!(lighting(
            material,
            light_point,
            position,
            eye_normal,
            surface_normal,
            false
        )
        .equal(Color::new(0.7364, 0.7364, 0.7364)));
    }

    #[test]
    fn lighting_light_opposite_offset() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal =
            Coordinates::new_vector(0.0, -(2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert!(lighting(
            material,
            light_point,
            position,
            eye_normal,
            surface_normal,
            false
        )
        .equal(Color::new(1.6364, 1.6364, 1.6364)));
    }

    #[test]
    fn lighting_light_behind_surface() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 0.0, 10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            lighting(
                material,
                light_point,
                position,
                eye_normal,
                surface_normal,
                false
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_in_shadow() {
        let material = Material::default();
        let position = Coordinates::new_point(0.0, 0.0, 0.0);
        let eye_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let surface_normal = Coordinates::new_vector(0.0, 0.0, -1.0);
        let light_point = Light::new(
            Coordinates::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            lighting(
                material,
                light_point,
                position,
                eye_normal,
                surface_normal,
                true
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }
}
