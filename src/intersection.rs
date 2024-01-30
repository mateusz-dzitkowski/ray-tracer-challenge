use crate::sphere::Sphere;
use crate::types::Field;

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
