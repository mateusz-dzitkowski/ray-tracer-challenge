mod canvas;
mod colour;
mod intersection;
mod ray;
mod sphere;
mod types;

use canvas::Canvas;
use num_traits::FloatConst;
use ray::Ray;
use sphere::Sphere;
use types::*;

const RAY_ORIGIN: Point = Point::new(0., 0., -5.);
const WALL_Z: f32 = 10.;
const WALL_SIZE: usize = 10;
const HALF_WALL_SIZE: f32 = WALL_SIZE as f32 / 2.;
const CANVAS_PIXELS: usize = 1000;
const PIXEL_SIZE: f32 = WALL_SIZE as f32 / CANVAS_PIXELS as f32;

fn main() {
    let shape = Sphere::new(Point::new(1., 1., 0.), 0.6);
    let colour = colour::red();
    let mut canvas: Canvas<CANVAS_PIXELS, CANVAS_PIXELS> = Canvas::default();

    for y in 0..CANVAS_PIXELS {
        let world_y = -HALF_WALL_SIZE + PIXEL_SIZE * y as f32;
        for x in 0..CANVAS_PIXELS {
            let world_x = -HALF_WALL_SIZE + PIXEL_SIZE * x as f32;
            let position = Point::new(world_x, world_y, WALL_Z);
            let ray = Ray::new(RAY_ORIGIN, (position - RAY_ORIGIN).normalize());
            if let Some(_) = ray.sphere_intersection(shape) {
                canvas.set(x, y, colour);
            }
        }
    }

    canvas.to_ppm_file("sphere.ppm");
}
