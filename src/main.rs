use nalgebra::{Point2, Vector2};

mod canvas;
mod colour;

use canvas::Canvas;

#[derive(Clone, Copy)]
struct Projectile {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Vector2<f32>,
    wind: Vector2<f32>,
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
        position: Point2::new(0., 1.),
        velocity: Vector2::new(1., 1.8).normalize() * 11.25,
    };

    let e = Environment {
        gravity: Vector2::new(0., -0.2),
        wind: Vector2::new(-0.01, 0.),
    };

    let mut c: Canvas<900, 550> = Canvas::default();

    while p.position.y >= 0. {
        tick(&mut p, &mut c, e);
    }

    c.to_ppm_file("canvas.ppm");
}
