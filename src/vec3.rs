use std::ops;

pub type Nm = f32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: Nm,
    pub y: Nm,
    pub z: Nm,
}

impl Vec3 {
    pub fn new(x: Nm, y: Nm, z: Nm) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> Nm {
        self.dots().sqrt()
    }

    pub fn dots(&self) -> Nm {
        self.dot(self)
    }

    pub fn dot(&self, o: &Self) -> Nm {
        self.x * o.x + self.y * o.y + self.z * o.z
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

impl ops::Add<Nm> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Nm) -> Self::Output {
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

impl ops::Mul<Nm> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Nm) -> Self::Output {
        self * Vec3::from(rhs)
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

impl ops::Div<Nm> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Nm) -> Self::Output {
        self / Vec3::from(rhs)
    }
}

impl From<Nm> for Vec3 {
    fn from(n: Nm) -> Self {
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
