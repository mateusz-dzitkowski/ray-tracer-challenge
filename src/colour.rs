use nalgebra::Point3;

pub type Colour = Point3<f32>;

pub fn colour_to_string(colour: Colour) -> String {
    let rgb: Vec<_> = colour
        .iter()
        .map(|colour| ((colour.max(0.).min(1.) * 255.).round() as u8).to_string())
        .collect();
    rgb.join(" ")
}

pub fn black() -> Colour {
    Colour::new(0., 0., 0.)
}
pub fn blue() -> Colour {
    Colour::new(0., 0., 1.)
}
pub fn green() -> Colour {
    Colour::new(0., 1., 0.)
}
pub fn cyan() -> Colour {
    Colour::new(0., 1., 1.)
}

pub fn red() -> Colour {
    Colour::new(1., 0., 0.)
}
pub fn magenta() -> Colour {
    Colour::new(1., 0., 1.)
}
pub fn yellow() -> Colour {
    Colour::new(1., 1., 0.)
}

pub fn white() -> Colour {
    Colour::new(1., 1., 1.)
}
