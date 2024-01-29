mod canvas;
mod colour;
mod types;

use canvas::Canvas;
use types::*;

#[derive(Clone, Copy)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick<const W: usize, const H: usize>(p: &mut Projectile, c: &mut Canvas<W, H>, e: Environment) {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + e.gravity + e.wind;

    let w_index = p.position.x as usize;
    let h_index = p.position.y as usize;
    let colour = colour::cyan();
    c.set(w_index, h_index, colour);
}

fn main() {
    let mut p = Projectile {
        position: Point::new(0., 1., 0.),
        velocity: Vector::new(1., 1.8, 0.).normalize() * 11.25,
    };

    let e = Environment {
        gravity: Vector::new(0., -0.2, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let mut c: Canvas<900, 550> = Canvas::default();

    while p.position.y >= 0. {
        tick(&mut p, &mut c, e);
    }

    c.to_ppm_file("canvas.ppm");
}
