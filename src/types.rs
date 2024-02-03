use nalgebra::{Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3};

pub type Field = f32;

pub type Point = Point3<Field>;

pub type Vector = Vector3<Field>;

pub type Matrix = Matrix4<Field>;

pub type Translation = Translation3<Field>;

pub type Scale = Scale3<Field>;

pub type Rotation = Rotation3<Field>;


pub fn reflect(incoming: Vector, normal: Vector) -> Vector {
    incoming - normal * 2. * incoming.dot(&normal)
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Vector::new(1., -1., 0.), Vector::new(0., 1., 0.), Vector::new(1., 1., 0.))]
    #[case(Vector::new(0., -1., 0.), Vector::new(2f32.sqrt() / 2., 2f32.sqrt() / 2., 0.), Vector::new(1., 0., 0.))]
    fn test_reflect(#[case] incoming: Vector, #[case] normal: Vector, #[case] expected: Vector) {
        assert_relative_eq!(reflect(incoming, normal), expected);
    }
}
