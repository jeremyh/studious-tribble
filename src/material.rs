use crate::hitable::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
    ) -> Option<Scatter>;
}

pub struct Lambertian {
    pub attenuation: Vec3,
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::random() * 2.0 - Vec3::ONE;

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &Hit,
    ) -> Option<Scatter> {
        let target: Vec3 = hit.p
            + hit.normal
            + random_in_unit_sphere();

        Some(Scatter {
            ray: Ray::new(hit.p, target - hit.p),
            attenuation: self.attenuation,
        })
    }
}
