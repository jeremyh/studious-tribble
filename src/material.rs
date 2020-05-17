use crate::hitable::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Scatter {
    Scattered { ray: Ray, attenuation: Vec3 },
    Stopped,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
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
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let target: Vec3 = hit.p
            + hit.normal
            + random_in_unit_sphere();

        Scatter::Scattered {
            ray: Ray::new(hit.p, target - hit.p),
            attenuation: self.albedo,
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz > 1. { 1. } else { fuzz },
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * v.dot(n)) * 2.
}

fn refract(
    v: &Vec3,
    n: &Vec3,
    ni_over_nt: f32,
) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0
        - ni_over_nt
            * ni_over_nt
            * ((dt * dt * -1.) + 1.);

    if discriminant > 0. {
        Some(
            (uv - *n * dt) * ni_over_nt
                - *n * discriminant.sqrt(),
        )
    } else {
        None
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let reflected =
            reflect(&ray.direction.unit(), &hit.normal);

        let scattered = Ray::new(
            hit.p,
            reflected
                + random_in_unit_sphere() * self.fuzz,
        );

        if scattered.direction.dot(&hit.normal) < 0. {
            return Scatter::Stopped;
        }

        Scatter::Scattered {
            ray: scattered,
            attenuation: self.albedo,
        }
    }
}

pub struct Dialectric {
    pub reflective_index: f32,
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let reflected =
            reflect(&ray.direction, &hit.normal);
        let attenuation = Vec3::ONE;
        let (outward_normal, ni_over_nt) =
            if ray.direction.dot(&hit.normal) > 0. {
                (-hit.normal, self.reflective_index)
            } else {
                (hit.normal, 1. / self.reflective_index)
            };

        if let Some(refracted) = refract(
            &ray.direction,
            &outward_normal,
            ni_over_nt,
        ) {
            Scatter::Scattered {
                ray: Ray::new(hit.p, refracted),
                attenuation,
            }
        } else {
            Scatter::Scattered {
                ray: Ray::new(hit.p, reflected),
                attenuation,
            }
        }
    }
}
