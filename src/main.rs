mod canvas;
mod colour;
mod intersection;
mod light;
mod material;
mod ray;
mod sphere;
mod types;

use crate::colour::Colour;
use crate::light::{lighting, LightSource};
use crate::material::Material;
use canvas::Canvas;
use num_traits::FloatConst;
use ray::Ray;
use sphere::Sphere;
use types::*;

const RAY_ORIGIN: Point = Point::new(0., 0., -5.);
const WALL_Z: f32 = 10.;
const WALL_SIZE: usize = 10;
const HALF_WALL_SIZE: f32 = WALL_SIZE as f32 / 2.;
const CANVAS_PIXELS: usize = 1500;
const PIXEL_SIZE: f32 = WALL_SIZE as f32 / CANVAS_PIXELS as f32;

fn main() {
    let colour = Colour::new(1., 0.2, 1.);
    let material = Material::new(colour, 0.1, 0.9, 0.9, 200.);
    let shape = Sphere::new(Point::new(0., 0., 0.), 1., material);

    let light_source = LightSource::sun_at(-10., 10., -10.);

    let mut canvas: Canvas<CANVAS_PIXELS, CANVAS_PIXELS> = Canvas::default();

    for y in 0..CANVAS_PIXELS {
        let world_y = -HALF_WALL_SIZE + PIXEL_SIZE * y as f32;
        for x in 0..CANVAS_PIXELS {
            let world_x = -HALF_WALL_SIZE + PIXEL_SIZE * x as f32;
            let position = Point::new(world_x, world_y, WALL_Z);
            let ray = Ray::new(RAY_ORIGIN, (position - RAY_ORIGIN).normalize());
            if let Some((intersection, _)) = ray.sphere_intersection(shape) {
                let point = ray.position(intersection.t);
                if let Some(normal) = intersection.object.normal_at(point) {
                    let eye = -ray.direction;
                    canvas.set(
                        x,
                        y,
                        lighting(
                            intersection.object.material,
                            light_source,
                            point,
                            eye,
                            normal,
                        ),
                    );
                }
            }
        }
    }

    canvas.to_ppm_file("sphere.ppm");
}
