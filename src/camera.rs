use crate::{ray::Ray, vec3::Vec3};

const PI: f32 = 3.141592;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}
impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        Camera {
            lower_left_corner: Vec3::new(
                -half_width,
                -half_height,
                -1.,
            ),
            horizontal: Vec3::new(
                2. * half_width,
                0.,
                0.,
            ),
            vertical: Vec3::new(
                0.,
                2. * half_height,
                0.,
            ),
            origin: Vec3::ZERO,
        }
    }
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin),
        }
    }
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
            origin: Vec3::ZERO,
        }
    }
}
