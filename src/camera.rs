use crate::{ray::Ray, vec3::Vec3};

use crate::vec3::{F, PI};

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,

    u_v_w: (Vec3, Vec3, Vec3),
    lens_radius: F,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: F,
        aspect: F,
        aperture: F,
        focus_disk: F,
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
                - half_width * focus_disk * u
                - half_height * focus_disk * v
                - focus_disk * w,
            horizontal: 2.
                * half_width
                * focus_disk
                * u,
            vertical: 2. * half_height * focus_disk * v,
            origin,
            u_v_w: (u, v, w),
            lens_radius: aperture / 2.,
        }
    }
    pub fn ray(&self, s: F, t: F) -> Ray {
        let rd =
            self.lens_radius * random_in_unit_disk();
        let offset =
            self.u_v_w.0 * rd.x + self.u_v_w.1 * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: (self.lower_left_corner
                + self.horizontal * s
                + self.vertical * t
                - self.origin
                - offset),
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let rand_disc = Vec3::new(
            rand::random(),
            rand::random(),
            0.,
        );
        let p = 2. * rand_disc - Vec3::new(1., 1., 0.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}
