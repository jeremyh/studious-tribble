use crate::{ray::Ray, vec3::Vec3};
use std::f32::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(&w);
        let v = w.cross(&u);

        let origin = look_from;

        Camera {
            lower_left_corner: origin
                - u * half_width
                - v * half_height
                - w,
            horizontal: u * 2. * half_width,
            vertical: v * 2. * half_height,
            origin,
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
