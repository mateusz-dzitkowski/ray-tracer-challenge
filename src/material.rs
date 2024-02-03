use crate::colour::{green, Colour};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Material {
    colour: Colour,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    pub fn new(colour: Colour, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            colour,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(green(), 1., 1., 1., 200.)
    }
}
