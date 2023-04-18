#[cfg(test)]
use std::cmp::PartialEq;

use super::{Color, Pattern};
use crate::base_types::Point;

pub struct Material {
    pub color: Color,
    pub patterns: Vec<Box<dyn Pattern>>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Color,
        patterns: Vec<Box<dyn Pattern>>,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Self {
        Self {
            color,
            patterns,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
        }
    }

    pub fn color_at(&self, point: Point) -> Color {
        let num_of_patterns = self.patterns.len();
        if num_of_patterns > 0 {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for pattern in self.patterns.iter() {
                let pattern_space_point =
                    pattern.get_transform().inverse().unwrap().coords_mul(point);
                color = color + pattern.color_at(pattern_space_point);
            }

            // blend
            return color / (num_of_patterns as f64);
        }

        self.color
    }
}

impl Default for Material {
    fn default() -> Self {
        // arbitrary default vals
        Self::new(
            Color(1.0, 1.0, 1.0),
            vec![],
            0.1,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
        )
    }
}

#[cfg(test)]
impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        let mut pattern_equal = true;
        for (i, _) in self.patterns.iter().enumerate() {
            // excellent robust check that patterns are equal :)
            let point_1 = Point::new_point(0.09, 10.1, -1.0);
            let point_2 = Point::new_point(3.11, 0.1, 57.0);
            if (self.patterns[i].color_at(point_1) != other.patterns[i].color_at(point_1))
                || (self.patterns[i].color_at(point_2) != other.patterns[i].color_at(point_2))
            {
                pattern_equal = false;
                break;
            }
        }

        pattern_equal
            && self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        let mut patterns = Vec::with_capacity(self.patterns.len());
        for pattern in self.patterns.iter() {
            patterns.push(pattern.clone_pattern());
        }

        Self {
            color: self.color,
            patterns,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            reflective: self.reflective,
            transparency: self.transparency,
            refractive_index: self.refractive_index,
        }
    }
}
