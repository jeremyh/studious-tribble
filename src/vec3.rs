use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.dot().sqrt()
    }

    pub fn dot(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + self.z * self.z)
    }
}
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
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
                z: 1.
            } + Vec3 {
                x: 2.,
                y: 2.,
                z: 2.
            },
            Vec3 {
                x: 3.,
                y: 3.,
                z: 3.
            }
        )
    }

    #[test]
    fn length() {
        assert_eq!(
            Vec3 {
                x: 6.,
                y: 3.,
                z: 7.
            }
            .length(),
            9.69536
        );
    }
}
