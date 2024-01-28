use crate::colour::{colour_to_string, Colour};
use std::fs;

pub struct Canvas<const W: usize, const H: usize>(Vec<Vec<Colour>>);

impl<const W: usize, const H: usize> Canvas<W, H> {
    pub fn get(self, w: usize, h: usize) -> Colour {
        self.0[h][w]
    }

    pub fn set(&mut self, w: usize, h: usize, colour: Colour) {
        if (w < W) & (h < H) {
            self.0[h][w] = colour;
        }
    }

    pub fn to_ppm(self) -> String {
        format!("P3\n{} {}\n255\n{}\n", W, H, String::from(self))
    }

    pub fn to_ppm_file(self, path: &str) {
        fs::write(path, self.to_ppm()).expect(format!("Unable to write to {}", path).as_str())
    }
}

impl<const W: usize, const H: usize> From<Canvas<W, H>> for String {
    fn from(value: Canvas<W, H>) -> Self {
        value
            .0
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(colour_to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .rev()
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<const W: usize, const H: usize> Default for Canvas<W, H> {
    fn default() -> Self {
        Self(vec![vec![Colour::default(); W]; H])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colour;
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
        let pixel: Colour = colour::green();
        blank.set(5, 10, pixel);
        assert_eq!(blank.get(5, 10), pixel);
    }

    #[rstest]
    fn test_blank_to_ppm(blank_small: Canvas<5, 3>) {
        let ppm = blank_small.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[rstest]
    fn test_to_ppm(mut blank_small: Canvas<5, 3>) {
        blank_small.set(0, 0, Colour::new(1.5, 0.0, 0.0));
        blank_small.set(2, 1, Colour::new(0.0, 0.5, 0.0));
        blank_small.set(4, 2, Colour::new(-0.5, 0., 1.0));
        let ppm = blank_small.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(lines.len(), 19);
        assert_eq!(lines[7], "0 0 255");
        assert_eq!(lines[10], "0 128 0");
        assert_eq!(lines[13], "255 0 0");
    }

    #[rstest]
    fn test_ends_with_newline(blank_small: Canvas<5, 3>) {
        assert!(blank_small.to_ppm().ends_with("\n"));
    }
}
