#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_ratio(
        r: f32,
        g: f32,
        b: f32,
    ) -> Color {
        fn to8(i: f32) -> u8 {
            (i * 255.99) as u8
        }

        Color {
            r: to8(r),
            g: to8(g),
            b: to8(b),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn conversion_from_ratio() {
        assert_eq!(
            Color::from_ratio(0.4, 1.0, 0.0),
            Color {
                r: 102,
                g: 255,
                b: 0,
            }
        )
    }
}
