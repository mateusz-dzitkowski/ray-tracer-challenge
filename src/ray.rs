use crate::intersection::Intersection;
use crate::sphere::Sphere;
use crate::types::{Field, Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: Field) -> Point {
        self.origin + self.direction * t
    }

    pub fn sphere_intersection(&self, sphere: Sphere) -> Option<(Intersection, Intersection)> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let Self {
            origin: o,
            direction: u,
        } = self;
        let Sphere {
            origin: c,
            radius: r,
            material: _,
        } = sphere;
        let a = u.norm_squared();
        let b = u.dot(&(o - c));
        let c = (o - c).norm_squared() - r.powi(2);
        let nabla = b.powi(2) - a * c;
        return if nabla < 0. {
            None
        } else {
            let first = (-b - nabla.sqrt()) / a;
            let second = (-b + nabla.sqrt()) / a;
            Some((
                Intersection::new(first, sphere),
                Intersection::new(second, sphere),
            ))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
        assert_eq!(&r.position(0.), &Point::new(2., 3., 4.));
        assert_eq!(&r.position(1.), &Point::new(3., 3., 4.));
        assert_eq!(&r.position(-1.), &Point::new(1., 3., 4.));
        assert_eq!(&r.position(2.5), &Point::new(4.5, 3., 4.));
    }

    #[rstest]
    fn test_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::default();
        let (first, second) = r.sphere_intersection(s).unwrap();
        assert_eq!(first.t, 4.);
        assert_eq!(second.t, 6.);
    }

    #[rstest]
    fn test_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::default();
        let (first, second) = r.sphere_intersection(s).unwrap();
        assert_eq!(first.t, 5.);
        assert_eq!(second.t, 5.);
    }

    #[rstest]
    fn test_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::default();
        assert!(r.sphere_intersection(s).is_none());
    }

    #[rstest]
    fn test_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::default();
        let (first, second) = r.sphere_intersection(s).unwrap();
        assert_eq!(first.t, -1.);
        assert_eq!(second.t, 1.);
    }

    #[rstest]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::default();
        let (first, second) = r.sphere_intersection(s).unwrap();
        assert_eq!(first.object, s);
        assert_eq!(second.object, s);
    }
}
