use derive_more::{Add, Sub, Neg};
use std::ops::{Div, Mul};
use nalgebra::Vector3;

#[derive(Add, Sub, Neg, Copy, Clone, PartialEq, Debug, Default)]
pub struct Colour(pub Vector3<f32>);

impl Colour {
    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vector3::new(r, g, b))
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

impl Div<f32> for Colour {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.0.x/rhs, self.0.y/rhs, self.0.z/rhs)
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.0.x * rhs.0.x,
            self.0.y * rhs.0.y,
            self.0.z * rhs.0.z,
        )
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0.x*rhs, self.0.y*rhs, self.0.z*rhs)
    }
}


#[cfg(test)]
mod tests {
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use super::*;
    use rstest::{fixture, rstest};
    use rand::{random, Rng};

    impl Distribution<Colour> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colour {
            Colour(Vector3::new_random())
        }
    }

    #[fixture]
    fn u() -> Colour {
        Colour(Vector3::new_random())
    }

    #[fixture]
    fn v() -> Colour {
        Colour(Vector3::new_random())
    }

    #[fixture]
    fn s() -> f32 {
        random()
    }

    #[rstest]
    fn test_add(u: Colour, v: Colour) {
        let w = u + v;
        assert_eq!(w.0.x, u.0.x + v.0.x);
        assert_eq!(w.0.y, u.0.y + v.0.y);
        assert_eq!(w.0.z, u.0.z + v.0.z);
    }

    #[rstest]
    fn test_sub(u: Colour, v: Colour) {
        let w = u - v;
        assert_eq!(w.0.x, u.0.x - v.0.x);
        assert_eq!(w.0.y, u.0.y - v.0.y);
        assert_eq!(w.0.z, u.0.z - v.0.z);
    }

    #[rstest]
    fn test_mul(u: Colour, v: Colour) {
        let w = u * v;
        assert_eq!(w.0.x, u.0.x * v.0.x);
        assert_eq!(w.0.y, u.0.y * v.0.y);
        assert_eq!(w.0.z, u.0.z * v.0.z);
    }

    #[rstest]
    fn test_negate(u: Colour) {
        let w = -u;
        assert_eq!(w.0.x, -u.0.x);
        assert_eq!(w.0.y, -u.0.y);
        assert_eq!(w.0.z, -u.0.z);
    }

    #[rstest]
    fn test_mul_by_scalar(u: Colour, s: f32) {
        let w = u * s;
        assert_eq!(w.0.x, u.0.x * s);
        assert_eq!(w.0.y, u.0.y * s);
        assert_eq!(w.0.z, u.0.z * s);
    }

    #[rstest]
    fn test_div_by_scalar(u: Colour, s: f32) {
        let w = u / s;
        assert_eq!(w.0.x, u.0.x / s);
        assert_eq!(w.0.y, u.0.y / s);
        assert_eq!(w.0.z, u.0.z / s);
    }

    #[rstest]
    #[case(Colour::new(1., 0., 0.), 1.)]
    #[case(Colour::new(0., 1., 0.), 1.)]
    #[case(Colour::new(0., 0., 1.), 1.)]
    #[case(Colour::new(0., 3., 4.), 5.)]
    #[case(Colour::new(0., -3., -4.), 5.)]
    fn test_norm(
        #[case] v: Colour,
        #[case] expected: f32,
    ) {
        assert_eq!(v.norm(), expected);
    }

    #[rstest]
    #[case(Colour::new(1., 0., 0.), Colour::new(1., 0., 0.))]
    #[case(Colour::new(4., 0., 0.), Colour::new(1., 0., 0.))]
    #[case(Colour::new(1., 2., 3.), Colour::new(1./14_f32.sqrt(), 2./14_f32.sqrt(), 3./14_f32.sqrt()))]
    fn test_normalize(
        #[case] v: Colour,
        #[case] w: Colour,
    ) {
        assert_eq!(v.normalize(), w);
    }

    #[rstest]
    #[case(Colour::new(1., 0., 0.), Colour::new(1., 0., 0.), 1.)]
    #[case(Colour::new(1., 0., 0.), Colour::new(0., 1., 1.), 0.)]
    #[case(Colour::new(1., 2., 3.), Colour::new(2., 3., 4.), 20.)]
    fn test_dot(
        #[case] v: Colour,
        #[case] w: Colour,
        #[case] expected: f32,
    ) {
        assert_eq!(v.dot(&w), expected);
    }

    #[rstest]
    #[case(Colour::new(1., 0., 0.), Colour::new(0., 1., 0.), Colour::new(0., 0., 1.))]
    #[case(Colour::new(0., 1., 0.), Colour::new(1., 0., 0.), Colour::new(0., 0., -1.))]
    #[case(Colour::new(1., 2., 3.), Colour::new(2., 3., 4.), Colour::new(-1., 2., -1.))]
    #[case(Colour::new(2., 3., 4.), Colour::new(1., 2., 3.), Colour::new(1., -2., 1.))]
    fn test_cross(
        #[case] v: Colour,
        #[case] w: Colour,
        #[case] expected: Colour,
    ) {
        assert_eq!(v.cross(&w), expected);
    }
}
