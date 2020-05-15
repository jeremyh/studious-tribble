use crate::vec3::Vec3;
use core::ops;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const SKY_BLUE: Color = Color {
        r: 0.5,
        g: 0.75,
        b: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.,
        b: 0.,
    };

    pub fn linear(
        start: Color,
        end: Color,
        t: f32,
    ) -> Color {
        let start: Vec3 = Color::into(start);
        let end: Vec3 = Color::into(end);

        Color::from(start * (1. - t) + end * t)
    }

    pub fn web_color(&self) -> (u8, u8, u8) {
        let gamma_correct =
            |i: f32| (i.powf(1.0 / 1.8));
        let to8 = |i: f32| (i * 255.99) as u8;

        (to8(self.r), to8(self.g), to8(self.b))
    }

    pub fn darken(&self, a: f32) -> Self {
        Color {
            r: self.r / a,
            g: self.g / a,
            b: self.b / a,
        }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }
}

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
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
            (102, 255, 0)
        )
    }
}
