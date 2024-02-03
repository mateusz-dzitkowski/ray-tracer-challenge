use crate::material::Material;
use crate::types::{Field, Point, Vector};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: Field,
    pub material: Material,
}

impl Sphere {
    pub fn new(origin: Point, radius: Field, material: Material) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }

    pub fn normal_at(&self, point: Point) -> Option<Vector> {
        if point != self.origin {
            Some((point - self.origin).normalize())
        } else {
            None
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Point::default(), 1., Material::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn sphere() -> Sphere {
        Sphere::new(Point::new(1., 0., 0.), 1., Material::default())
    }

    #[rstest]
    #[case(Point::new(2., 0., 0.), Vector::new(1., 0., 0.))]
    #[case(Point::new(0., 0., 0.), Vector::new(-1., 0., 0.))]
    #[case(Point::new(1., 1., 0.), Vector::new(0., 1., 0.))]
    #[case(Point::new(1., -1., 0.), Vector::new(0., -1., 0.))]
    #[case(Point::new(1., 0., 1.), Vector::new(0., 0., 1.))]
    #[case(Point::new(1., 0., -1.), Vector::new(0., 0., -1.))]
    fn test_normal_at(sphere: Sphere, #[case] point: Point, #[case] expected: Vector) {
        assert_relative_eq!(sphere.normal_at(point).unwrap(), expected);
    }

    #[rstest]
    fn test_normal_at_origin(sphere: Sphere) {
        assert!(sphere.normal_at(Point::new(1., 0., 0.)).is_none());
    }

    #[rstest]
    fn test_normal_at_is_normalized(sphere: Sphere) {
        let normal_at = sphere
            .normal_at(Point::new(3f32.sqrt(), 3f32.sqrt(), 3f32.sqrt()))
            .unwrap();
        assert_relative_eq!(normal_at, normal_at.normalize());
    }
}
