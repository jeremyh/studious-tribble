use crate::hitable::Sphere;
use crate::material::{Dialectric, Lambertian, Metal};
use crate::scene::Scene;
use crate::vec3::{randf, Vec3};
use crate::vec3::{F, PI};

pub(crate) fn random_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add(Box::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    }));

    scene.add(Box::new(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: Box::new(Dialectric {
            reflective_index: 1.5,
        }),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: Box::new(Metal::new(
            Vec3::new(0.7, 0.6, 0.5),
            0.0,
        )),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: F = randf();
            let center = Vec3::new(
                (a as F) + 0.9 * randf(),
                0.2,
                (b as F) + 0.9 * randf(),
            );

            if (center - Vec3::new(4., 0.2, 0.))
                .length()
                > 0.9
            {
                scene.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: if choose_mat < 0.8 {
                        Box::new(rand::random::<Lambertian>())
                    } else if choose_mat < 0.95 {
                        Box::new(rand::random::<Metal>())
                    } else {
                        // glass
                        Box::new(Dialectric {
                            reflective_index: 1.5,
                        })
                    },
                }));
            }
        }
    }

    scene
}
fn camera_test_scene() -> Scene {
    let r = (PI / 4.).cos();
    let mut scene = Scene::new();

    scene.add(Box::new(Sphere {
        center: Vec3::new(-r, 0., -1.),
        radius: r,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.1, 0.1, 0.3),
        }),
    }));

    scene.add(Box::new(Sphere {
        center: Vec3::new(r, 0., -1.),
        radius: r,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.3, 0.1, 0.1),
        }),
    }));

    scene
}

fn standard_scene() -> Scene {
    // Create scene
    let mut scene = Scene::new();

    scene.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        }),
    }));

    scene.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.),
        }),
    }));

    scene.add(Box::new(Sphere {
        center: Vec3::new(1., 0., -1.),
        radius: 0.5,
        material: Box::new(Metal::new(
            Vec3::new(0.8, 0.6, 0.2),
            0.0,
        )),
    }));

    scene.add(Box::new(Sphere {
        center: Vec3::new(-1., 0., -1.),
        radius: 0.5,
        material: Box::new(Dialectric {
            reflective_index: 1.5,
        }),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::new(-1., 0., -1.),
        radius: -0.45,
        material: Box::new(Dialectric {
            reflective_index: 1.5,
        }),
    }));

    scene
}
