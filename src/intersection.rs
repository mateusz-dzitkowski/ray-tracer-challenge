use crate::sphere::Sphere;
use crate::types::Field;

pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn hit(&self) -> Option<Intersection> {
        self.intersections
            .clone()
            .into_iter()
            .filter(|&intersection| intersection.t >= 0.)
            .collect::<Vec<_>>()
            .first()
            .copied()
    }
}

impl From<Vec<Intersection>> for Intersections {
    fn from(value: Vec<Intersection>) -> Self {
        let mut intersections = value;
        intersections.sort_by(|&x, y| x.t.partial_cmp(&y.t).unwrap());
        Self { intersections }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Intersection {
    pub t: Field,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: Field, object: Sphere) -> Self {
        Self { t, object }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn s() -> Sphere {
        Sphere::default()
    }

    #[fixture]
    fn i_minus2(s: Sphere) -> Intersection {
        Intersection::new(-2., s)
    }

    #[fixture]
    fn i_minus1(s: Sphere) -> Intersection {
        Intersection::new(-1., s)
    }

    #[fixture]
    fn i_1(s: Sphere) -> Intersection {
        Intersection::new(1., s)
    }

    #[fixture]
    fn i_2(s: Sphere) -> Intersection {
        Intersection::new(2., s)
    }

    #[rstest]
    fn test_all_intersections_have_positive_t(i_1: Intersection, i_2: Intersection) {
        let intersections = Intersections::from(vec![i_2, i_1]);
        assert_eq!(intersections.hit(), Some(i_1));
    }

    #[rstest]
    fn test_some_intersections_have_negative_t(i_1: Intersection, i_minus1: Intersection) {
        let intersections = Intersections::from(vec![i_minus1, i_1]);
        assert_eq!(intersections.hit(), Some(i_1));
    }

    #[rstest]
    fn test_all_intersections_have_negative_t(i_minus2: Intersection, i_minus1: Intersection) {
        let intersections = Intersections::from(vec![i_minus2, i_minus1]);
        assert!(intersections.hit().is_none());
    }

    #[rstest]
    fn test_hit_is_lowest_nonnegative_intersection(
        i_minus2: Intersection,
        i_minus1: Intersection,
        i_1: Intersection,
        i_2: Intersection,
    ) {
        let intersections = Intersections::from(vec![i_minus2, i_minus1, i_2, i_1]);
        assert_eq!(intersections.hit(), Some(i_1));
    }
}
