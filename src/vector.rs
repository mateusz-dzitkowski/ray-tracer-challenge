use derive_more::{Add, Sub, Neg};
use std::ops::{Div, Mul};
use nalgebra::Vector3;

#[derive(Add, Sub, Neg, Copy, Clone, PartialEq, Debug, Default)]
pub struct Vector(Vector3<f32>);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }

    pub fn norm(self) -> f32 {
        self.0.norm()
    }

    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }

    pub fn dot(self, rhs: &Self) -> f32 {
        self.0.dot(&rhs.0)
    }

    pub fn cross(self, rhs: &Self) -> Self {
        Self(self.0.cross(&rhs.0))
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.0.x/rhs, self.0.y/rhs, self.0.z/rhs)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.0.x * rhs.0.x,
            self.0.y * rhs.0.y,
            self.0.z * rhs.0.z,
        )
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0.x*rhs, self.0.y*rhs, self.0.z*rhs)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use rand::random;

    #[fixture]
    fn u() -> Vector {
        Vector(Vector3::new_random())
    }

    #[fixture]
    fn v() -> Vector {
        Vector(Vector3::new_random())
    }

    #[fixture]
    fn s() -> f32 {
        random()
    }

    #[rstest]
    fn test_add(u: Vector, v: Vector) {
        let w = u + v;
        assert_eq!(w.0.x, u.0.x + v.0.x);
        assert_eq!(w.0.y, u.0.y + v.0.y);
        assert_eq!(w.0.z, u.0.z + v.0.z);
    }

    #[rstest]
    fn test_sub(u: Vector, v: Vector) {
        let w = u - v;
        assert_eq!(w.0.x, u.0.x - v.0.x);
        assert_eq!(w.0.y, u.0.y - v.0.y);
        assert_eq!(w.0.z, u.0.z - v.0.z);
    }

    #[rstest]
    fn test_mul(u: Vector, v: Vector) {
        let w = u * v;
        assert_eq!(w.0.x, u.0.x * v.0.x);
        assert_eq!(w.0.y, u.0.y * v.0.y);
        assert_eq!(w.0.z, u.0.z * v.0.z);
    }

    #[rstest]
    fn test_negate(u: Vector) {
        let w = -u;
        assert_eq!(w.0.x, -u.0.x);
        assert_eq!(w.0.y, -u.0.y);
        assert_eq!(w.0.z, -u.0.z);
    }

    #[rstest]
    fn test_mul_by_scalar(u: Vector, s: f32) {
        let w = u * s;
        assert_eq!(w.0.x, u.0.x * s);
        assert_eq!(w.0.y, u.0.y * s);
        assert_eq!(w.0.z, u.0.z * s);
    }

    #[rstest]
    fn test_div_by_scalar(u: Vector, s: f32) {
        let w = u / s;
        assert_eq!(w.0.x, u.0.x / s);
        assert_eq!(w.0.y, u.0.y / s);
        assert_eq!(w.0.z, u.0.z / s);
    }

    #[rstest]
    #[case(Vector::new(1., 0., 0.), 1.)]
    #[case(Vector::new(0., 1., 0.), 1.)]
    #[case(Vector::new(0., 0., 1.), 1.)]
    #[case(Vector::new(0., 3., 4.), 5.)]
    #[case(Vector::new(0., -3., -4.), 5.)]
    fn test_norm(
        #[case] v: Vector,
        #[case] expected: f32,
    ) {
        assert_eq!(v.norm(), expected);
    }

    #[rstest]
    #[case(Vector::new(1., 0., 0.), Vector::new(1., 0., 0.))]
    #[case(Vector::new(4., 0., 0.), Vector::new(1., 0., 0.))]
    #[case(Vector::new(1., 2., 3.), Vector::new(1./14_f32.sqrt(), 2./14_f32.sqrt(), 3./14_f32.sqrt()))]
    fn test_normalize(
        #[case] v: Vector,
        #[case] w: Vector,
    ) {
        assert_eq!(v.normalize(), w);
    }

    #[rstest]
    #[case(Vector::new(1., 0., 0.), Vector::new(1., 0., 0.), 1.)]
    #[case(Vector::new(1., 0., 0.), Vector::new(0., 1., 1.), 0.)]
    #[case(Vector::new(1., 2., 3.), Vector::new(2., 3., 4.), 20.)]
    fn test_dot(
        #[case] v: Vector,
        #[case] w: Vector,
        #[case] expected: f32,
    ) {
        assert_eq!(v.dot(&w), expected);
    }

    #[rstest]
    #[case(Vector::new(1., 0., 0.), Vector::new(0., 1., 0.), Vector::new(0., 0., 1.))]
    #[case(Vector::new(0., 1., 0.), Vector::new(1., 0., 0.), Vector::new(0., 0., -1.))]
    #[case(Vector::new(1., 2., 3.), Vector::new(2., 3., 4.), Vector::new(-1., 2., -1.))]
    #[case(Vector::new(2., 3., 4.), Vector::new(1., 2., 3.), Vector::new(1., -2., 1.))]
    fn test_cross(
        #[case] v: Vector,
        #[case] w: Vector,
        #[case] expected: Vector,
    ) {
        assert_eq!(v.cross(&w), expected);
    }
}
