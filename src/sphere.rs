use crate::types::{Field, Point};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: Field,
}

impl Sphere {
    pub fn new(origin: Point, radius: Field) -> Self {
        Self { origin, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Point::default(), 1.)
    }
}
