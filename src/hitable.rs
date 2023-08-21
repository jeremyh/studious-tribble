use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, F};
use std::ops::Range;

pub struct Hit<'a> {
    pub t: F,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a (dyn Material + Send + Sync),
}

pub trait Hitable {
    fn hit(
        &self,
        ray: &Ray,
        t: &Range<F>,
    ) -> Option<Hit>;
}

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: F,
    pub material: Box<dyn Material + Send + Sync + 'a>,
}

impl Hitable for Sphere<'_> {
    /// Does the ray hit our sphere?
    /// If so, return the time t of the hit.
    fn hit(
        &self,
        ray: &Ray,
        within_t: &Range<F>,
    ) -> Option<Hit> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = oc.dot(&ray.direction);
        let c = oc.squared_length()
            - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant <= 0. {
            return None;
        }

        let hit_t = |t: F| {
            if within_t.contains(&t) {
                let p = ray.point_at(t);
                return Some(Hit {
                    t,
                    p,
                    normal: (p - self.center)
                        / self.radius,
                    material: self.material.as_ref(),
                });
            }
            None
        };

        let t = (-b - (b * b - a * c).sqrt()) / a;
        if let Some(h) = hit_t(t) {
            return Some(h);
        }

        let t = (-b + (b * b - a * c).sqrt()) / a;
        if let Some(h) = hit_t(t) {
            return Some(h);
        }

        None
    }
}
