use crate::vec3::{Vec3, F};
use core::ops;
use std::iter;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    v: Vec3,
}

#[derive(PartialEq, Debug, Eq, Copy, Clone, Default)]
pub struct WebColor(pub u8, pub u8, pub u8);

impl Color {
    pub fn new(r: F, g: F, b: F) -> Color {
        Self {
            v: Vec3::new(r, g, b),
        }
    }

    pub fn white() -> Color {
        Color::new(1., 1., 1.)
    }
    pub fn black() -> Color {
        Color::new(0., 0., 0.)
    }
    pub fn sky_blue() -> Color {
        Color::new(0.5, 0.75, 1.)
    }
    pub fn red() -> Color {
        Color::new(1., 0., 0.)
    }

    pub fn linear(
        start: Color,
        end: Color,
        t: F,
    ) -> Color {
        let start: Vec3 = Color::into(start);
        let end: Vec3 = Color::into(end);

        Color::from(start * (1. - t) + end * t)
    }

    pub fn web_color(&self) -> WebColor {
        let gamma_correct = |i: F| (i.powf(1.0 / 1.8));
        let to8 =
            |i: F| (gamma_correct(i) * 255.99) as u8;

        WebColor(
            to8(self.v.x),
            to8(self.v.y),
            to8(self.v.z),
        )
    }

    pub fn darken(&self, a: F) -> Self {
        Color { v: self.v / a }
    }

    pub fn attenuate(&self, v: Vec3) -> Self {
        Color { v: self.v * v }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color { v }
    }
}

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        self.v
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color { v: self.v + rhs.v }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.v += rhs.v
    }
}

impl iter::Sum for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_from_ratio() {
        assert_eq!(
            Color::from(Vec3::new(0.4, 1.0, 0.0))
                .web_color(),
            // Without Gamma correction:
            // (102, 255, 0)
            // With gamma:
            WebColor(153, 255, 0)
        )
    }
}
