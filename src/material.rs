use crate::hitable::Hit;
use crate::ray::Ray;
use crate::vec3::{randf, Vec3, F};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

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
    fuzz: F,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: F) -> Self {
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
    ni_over_nt: F,
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
    pub reflective_index: F,
}

pub fn schlick(cosine: F, reflective_index: F) -> F {
    let r0: F = ((1. - reflective_index)
        / (1. + reflective_index))
        .powi(2);

    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Scatter {
        let reflected =
            reflect(&ray.direction, &hit.normal);
        let attenuation = Vec3::ONE;

        let (outward_normal, ni_over_nt, cosine) =
            if ray.direction.dot(&hit.normal) > 0. {
                (
                    -hit.normal,
                    self.reflective_index,
                    self.reflective_index
                        * ray
                            .direction
                            .dot(&hit.normal)
                        / ray.direction.length(),
                )
            } else {
                (
                    hit.normal,
                    1. / self.reflective_index,
                    -ray.direction.dot(&hit.normal)
                        / ray.direction.length(),
                )
            };

        let direction = if let Some(refracted) = refract(
            &ray.direction,
            &outward_normal,
            ni_over_nt,
        ) {
            if randf()
                < schlick(cosine, self.reflective_index)
            {
                reflected
            } else {
                refracted
            }
        } else {
            reflected
        };

        Scatter::Scattered {
            ray: Ray::new(hit.p, direction),
            attenuation,
        }
    }
}

impl Distribution<Metal> for Standard {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Metal {
        let mut r =
            || -> F { 0.5 * (1. + rng.gen::<F>()) };

        Metal {
            albedo: Vec3::new(r(), r(), r()),
            fuzz: r(),
        }
    }
}

impl Distribution<Lambertian> for Standard {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Lambertian {
        let mut r =
            || -> F { rng.gen::<F>() * rng.gen::<F>() };

        Lambertian {
            albedo: Vec3::new(r(), r(), r()),
        }
    }
}
