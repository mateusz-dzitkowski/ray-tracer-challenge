use crate::colour::Colour;

struct Canvas<const W: usize, const H: usize>([[Colour; H]; W]);

impl<const W: usize, const H: usize> Canvas<W, H> {
    pub fn get(self, i: usize, j: usize) -> Colour {
        self.0[i][j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut Colour {
        &mut self.0[i][j]
    }

    pub fn encode(self) -> String {
        format!("P3\n{} {}\n255", self.0.len(), self.0[0].len())
    }
}

impl<const W: usize, const H: usize> Default for Canvas<W, H> {
    fn default() -> Self {
        Self([[Colour::default(); H]; W])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn blank() -> Canvas<10, 20> {
        Canvas::default()
    }

    #[fixture]
    fn blank_small() -> Canvas<5, 3> {
        Canvas::default()
    }

    #[rstest]
    fn test_write_pixel_to_canvas(mut blank: Canvas<10, 20>) {
        let pixel: Colour = Colour::green();
        *blank.get_mut(5, 10) = pixel;
        assert_eq!(blank.get(5, 10), pixel);
    }

    #[rstest]
    fn test_to_ppm(blank: Canvas<10, 20>) {
        assert_eq!(blank.encode(), "P3\n10 20\n255".to_string())
    }

    #[rstest]
    fn test_encode(mut blank_small: Canvas<5, 3>) {
        *blank_small.get_mut(0, 0) = Colour::new(1.5, 0.0, 0.0);
        *blank_small.get_mut(0, 1) = Colour::new(0.0, 0.5, 0.0);
        *blank_small.get_mut(0, 2) = Colour::new(-0.5, 0., 1.0);
        assert_eq!(blank_small.encode(), "".to_string());
    }
}
