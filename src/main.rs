mod canvas;
mod colour;
mod types;
mod ray;
mod sphere;
mod intersection;

use canvas::Canvas;
use num_traits::FloatConst;
use types::*;

fn main() {
    let mut canvas: Canvas<51, 51> = Canvas::default();
    let c = colour::red();
    for n in 0..12 {
        let p = Point::new(20., 0., 0.);
        let rotation = Rotation::new(Vector::new(0., 0., f32::PI() * n as f32 / 6.));
        let translation = Translation::new(25., 25., 0.);
        let w = translation * rotation * p;
        let i = w.x.round() as usize;
        let j = w.y.round() as usize;
        canvas.set(i, j, c);
    }
    canvas.to_ppm_file("clock.ppm");
}
