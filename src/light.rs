use crate::colour::{white, Colour};
use crate::types::Field;
use crate::Point;

pub struct PointLight {
    origin: Point,
    colour: Colour,
}

impl PointLight {
    pub fn new(origin: Point, colour: Colour) -> Self {
        Self { origin, colour }
    }

    pub fn sun_at(x: Field, y: Field, z: Field) -> Self {
        Self::new(Point::new(x, y, z), white())
    }
}
