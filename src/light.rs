use crate::colour::{black, white, Colour};
use crate::material::Material;
use crate::types::{reflect, Field, Point, Scale, Vector};

pub struct LightSource {
    origin: Point,
    colour: Colour,
}

impl LightSource {
    pub fn new(origin: Point, colour: Colour) -> Self {
        Self { origin, colour }
    }

    pub fn sun_at(x: Field, y: Field, z: Field) -> Self {
        Self::new(Point::new(x, y, z), white())
    }
}

pub fn lighting(
    material: Material,
    light_source: LightSource,
    point: Point,
    eye_normal: Vector,
    surface_normal: Vector,
) -> Colour {
    let effective_colour = Scale::from(light_source.colour) * material.colour;
    let light_normal = (light_source.origin - point).normalize();
    let ambient = effective_colour * material.ambient;

    let light_dot_surface = light_normal.dot(&surface_normal);
    if light_dot_surface < 0. {
        return ambient; // light is on the other side of the surface - no diffuse light
    }

    let diffuse = effective_colour * material.diffuse * light_dot_surface;

    let reflection_dot_eye = reflect(-light_normal, surface_normal).dot(&eye_normal);
    if reflection_dot_eye <= 0. {
        return ambient + Vector::from(diffuse.coords); // light reflects away from the eye - no specular light
    }

    let factor = reflection_dot_eye.powf(material.shininess);
    let specular = light_source.colour * material.specular * factor;

    ambient + Vector::from(diffuse.coords) + Vector::from(specular.coords)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn material() -> Material {
        Material::default()
    }

    #[fixture]
    fn point() -> Point {
        Point::default()
    }

    #[fixture]
    fn surface_normal() -> Vector {
        -Vector::z()
    }

    #[rstest]
    #[case::eye_between_the_light_and_the_surface(-Vector::z(), LightSource::sun_at(0., 0., -10.), white() * 1.9)]
    #[case::eye_offset_45_degrees(Vector::new(0., 1., -1.).normalize(), LightSource::sun_at(0., 0., -10.), white())]
    #[case::light_offset_45_degrees(-Vector::z().normalize(), LightSource::sun_at(0., 10., -10.), white() * 0.7364)]
    #[case::eye_in_the_path_of_reflestion_vector(Vector::new(0., -1., -1.).normalize(), LightSource::sun_at(0., 10., -10.), white() * 1.6364)]
    #[case::light_behind_the_surface(Vector::new(0., 0., -1.).normalize(), LightSource::sun_at(0., 0., 10.), white() * 0.1)]
    fn test_lighting(
        material: Material,
        point: Point,
        surface_normal: Vector,
        #[case] eye_normal: Vector,
        #[case] light_source: LightSource,
        #[case] expected: Colour,
    ) {
        assert_relative_eq!(
            lighting(material, light_source, point, eye_normal, surface_normal),
            Colour::new(1.9, 1.9, 1.9)
        );
    }
}
