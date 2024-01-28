use derive_more::{Add, Sub};
use nalgebra::Vector3;
use std::ops::{Div, Mul};

#[derive(Add, Sub, Copy, Clone, PartialEq, Debug, Default)]
pub struct Colour(pub Vector3<f32>);

impl Colour {
    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }
    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }
    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }
    pub fn cyan() -> Self {
        Self::new(0., 1., 1.)
    }

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }
    pub fn magenta() -> Self {
        Self::new(1., 0., 1.)
    }
    pub fn yellow() -> Self {
        Self::new(1., 1., 0.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vector3::new(r, g, b))
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0.x * rhs.0.x, self.0.y * rhs.0.y, self.0.z * rhs.0.z)
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0.x * rhs, self.0.y * rhs, self.0.z * rhs)
    }
}

impl From<Colour> for String {
    fn from(colour: Colour) -> String {
        let rgb: Vec<_> = colour
            .0
            .iter()
            .map(|colour| ((colour.max(0.).min(1.) * 255.).round() as u8).to_string())
            .collect();
        rgb.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use rstest::{fixture, rstest};

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
        assert_eq!(
            u + v,
            Colour::new(u.0.x + v.0.x, u.0.y + v.0.y, u.0.z + v.0.z)
        );
    }

    #[rstest]
    fn test_sub(u: Colour, v: Colour) {
        assert_eq!(
            u - v,
            Colour::new(u.0.x - v.0.x, u.0.y - v.0.y, u.0.z - v.0.z)
        );
    }

    #[rstest]
    fn test_mul(u: Colour, v: Colour) {
        assert_eq!(
            u * v,
            Colour::new(u.0.x * v.0.x, u.0.y * v.0.y, u.0.z * v.0.z)
        );
    }

    #[rstest]
    #[case(Colour::red(), "255 0 0".into())]
    #[case(Colour::green(), "0 255 0".into())]
    #[case(Colour::blue(), "0 0 255".into())]
    #[case(Colour::new(-1., 0.5, 1.5), "0 128 255".into())]
    fn test_into_string(#[case] colour: Colour, #[case] expected: String) {
        assert_eq!(String::from(colour), expected)
    }
}
