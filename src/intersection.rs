use crate::types::Field;
use crate::sphere::Sphere;

#[derive(PartialEq)]
pub struct Intersection {
    pub t: Field,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: Field, object: Sphere) -> Self {
        Self { t, object }
    }
}
