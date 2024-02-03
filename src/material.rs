use crate::colour::{white, Colour};
use crate::types::Field;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Material {
    pub colour: Colour,
    pub ambient: Field,
    pub diffuse: Field,
    pub specular: Field,
    pub shininess: Field,
}

impl Material {
    pub fn new(
        colour: Colour,
        ambient: Field,
        diffuse: Field,
        specular: Field,
        shininess: Field,
    ) -> Self {
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
        Self::new(white(), 0.1, 0.9, 0.9, 200.)
    }
}
