use std::ops;

// Should we render with f32s or f64s?
pub type F = f32;
pub const PI: F = std::f64::consts::PI as F;

/// Rand between 0-1. Centralised so that I could experiment with different rand functions easily..
pub fn randf() -> F {
    rand::random()
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    pub fn new(x: F, y: F, z: F) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random() -> Self {
        Self::new(randf(), randf(), randf())
    }
    pub fn length(&self) -> F {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> F {
        self.dot(self)
    }

    pub fn dot(&self, b: &Self) -> F {
        let a = self;
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(&self, b: &Self) -> Self {
        let a = self;
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
    pub fn unit(&self) -> Self {
        let len = self.length();

        *self / len
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<F> for Vec3 {
    type Output = Self;
    fn add(self, rhs: F) -> Self::Output {
        self + Vec3::from(rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
        )
    }
}

impl ops::Mul<F> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        self * Vec3::from(rhs)
    }
}

impl ops::Mul<Vec3> for F {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
        )
    }
}

impl ops::Div<F> for Vec3 {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        self / Vec3::from(rhs)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<F> for Vec3 {
    fn from(n: F) -> Self {
        Vec3::new(n, n, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(
            Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            } + Vec3 {
                x: 2.,
                y: 2.,
                z: 2.,
            },
            Vec3 {
                x: 3.,
                y: 3.,
                z: 3.,
            }
        )
    }

    #[test]
    fn length() {
        assert_eq!(
            Vec3 {
                x: 6.,
                y: 3.,
                z: 7.,
            }
            .length(),
            9.69536
        );
    }

    #[test]
    fn units() {
        assert_eq!(
            Vec3::new(5., 6., 3.).unit(),
            Vec3 {
                x: 0.59761435,
                y: 0.71713716,
                z: 0.35856858,
            }
        )
    }
}
