#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use camera::Camera;
use color::Color;
use vec3::{F, PI};

use crate::hitable::{Hitable, Sphere};
use crate::material::{
    Dialectric, Lambertian, Metal, Scatter,
};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use std::io;
use std::time::Instant;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod scene;
mod vec3;

const MAX_DEPTH: i32 = 50;

fn color(
    ray: &Ray,
    scene: &dyn Hitable,
    depth: i32,
) -> Color {
    if let Some(hit) =
        scene.hit(&ray, &((0.001 as F)..F::INFINITY))
    {
        return if depth > MAX_DEPTH {
            Color::BLACK
        } else if let Scatter::Scattered {
            ray: scattered_ray,
            attenuation: scattered_attenuation,
        } = hit.material.scatter(ray, &hit)
        {
            color(&scattered_ray, scene, depth + 1)
                .attenuate(scattered_attenuation)
        } else {
            Color::BLACK
        };
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);

    Color::linear(Color::WHITE, Color::SKY_BLUE, t)
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chambray",
    about = "Create a ray-traced image"
)]
struct Opt {
    #[structopt(short, long, default_value = "400")]
    width: i32,
    #[structopt(short, long, default_value = "200")]
    height: i32,

    #[structopt(
        parse(from_os_str),
        default_value = "image.ppm"
    )]
    output: PathBuf,

    #[structopt(long, default_value = "16")]
    samples: u16,
}

fn standard_scene<'a>() -> Scene<'a> {
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

fn camera_test_scene<'a>() -> Scene<'a> {
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

fn random_scene() -> Scene<'static> {
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
            let choose_mat: F = rand::random::<F>();
            let center = Vec3::new(
                (a as F) + 0.9 * rand::random::<F>(),
                0.2,
                (b as F) + 0.9 * rand::random::<F>(),
            );

            if (center - Vec3::new(4., 0.2, 0.))
                .length()
                > 0.9
            {
                scene.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material:

                    if choose_mat < 0.8 {
                        Box::new(rand::random::<Lambertian>())
                    } else if choose_mat < 0.95 {
                        Box::new(rand::random::<Metal>())
                    } else { // glass
                        Box::new(Dialectric { reflective_index: 1.5 })
                    },
                }));
            }
        }
    }

    scene
}

fn main() -> Result<(), anyhow::Error> {
    let opt: Opt = Opt::from_args();

    let scene = random_scene();
    let aspect = (opt.width as F) / (opt.height as F);
    println!(
        "Creating {}x{} with {} samples per pixel to {:?}",
        opt.width, opt.height, opt.samples, opt.output
    );

    let look_from = Vec3::new(12., 6., 0.51);
    let look_at = Vec3::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 1.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        20.,
        aspect,
        aperture,
        dist_to_focus,
    );

    render(
        &scene,
        &camera,
        opt.width,
        opt.height,
        opt.output.as_path(),
        opt.samples,
    )
}

fn render(
    scene: &Scene,
    camera: &Camera,
    width: i32,
    height: i32,
    path: &Path,
    samples: u16,
) -> Result<(), anyhow::Error> {
    let mut o = BufWriter::new(File::create(path)?);
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = width,
        ny = height
    )?;

    // TODO: Split out a separate image writer logic

    let start = Instant::now();
    for j in (0..height).rev() {
        // Print progress percentage.
        if j % 10 == 0 {
            let perc = 100.
                - ((j as f32) / (height as f32) * 100.)
                    .floor();
            print!("\r{:>3}% ", perc);
            io::stdout().flush().unwrap();
        }

        for i in 0..width {
            let mut color_samples = Color::BLACK;

            for s in 0..samples {
                let u = (i as F + rand::random::<F>())
                    / (width as F);
                let v = (j as F + rand::random::<F>())
                    / (height as F);
                let ray: Ray = camera.ray(u, v);

                color_samples += color(&ray, scene, 0);
            }

            {
                let (r, g, b) = color_samples
                    .darken(samples as F)
                    .web_color();

                writeln!(
                    &mut o,
                    "{:?} {:?} {:?}",
                    r, g, b,
                )?;
            }
        }
    }
    println!("\rDone in {:?}", start.elapsed());
    Ok(())
}
