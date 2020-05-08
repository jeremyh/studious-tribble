use crate::vec3::Vec3;
use std::ops::Mul;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const SKY_BLUE: Color = Color {
        r: 127,
        g: 178,
        b: 255,
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
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        let scale = |x: u8| ((x as f32) * rhs) as u8;

        Color {
            r: scale(self.r),
            g: scale(self.g),
            b: scale(self.b),
        }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        let to8 = |i: f32| (i * 255.99) as u8;

        Color {
            r: to8(v.x),
            g: to8(v.y),
            b: to8(v.z),
        }
    }
}

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        let from8 = |i: u8| (i as f32) / 255.99;
        Vec3::new(
            from8(self.r),
            from8(self.g),
            from8(self.b),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_from_ratio() {
        assert_eq!(
            Color::from(Vec3::new(0.4, 1.0, 0.0)),
            Color {
                r: 102,
                g: 255,
                b: 0,
            }
        )
    }
}
