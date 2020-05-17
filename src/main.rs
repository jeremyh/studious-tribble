#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use color::Color;
use vec3::Nm;

use crate::hitable::{Hitable, Sphere};
use crate::material::{
    Dialectric, Lambertian, Metal, Scatter,
};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use camera::Camera;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod scene;
mod vec3;

fn color(
    ray: &Ray,
    scene: &dyn Hitable,
    depth: i32,
) -> Color {
    if let Some(hit) =
        scene.hit(&ray, &(0.001f32..f32::INFINITY))
    {
        return if depth > 50 {
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

fn main() -> Result<(), anyhow::Error> {
    let opt: Opt = Opt::from_args();

    // Create scene
    let mut scene = Scene::new();

    let lambertian1 = Lambertian {
        albedo: Vec3::new(0.8, 0.3, 0.3),
    };
    scene.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
        material: &lambertian1,
    }));

    let lambertian2 = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    };
    scene.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
        material: &lambertian2,
    }));

    let metal =
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);
    scene.add(Box::new(Sphere {
        center: Vec3::new(1., 0., -1.),
        radius: 0.5,
        material: &metal,
    }));
    let dialectric = Dialectric {
        reflective_index: 1.5,
    };
    scene.add(Box::new(Sphere {
        center: Vec3::new(-1., 0., -1.),
        radius: 0.5,
        material: &dialectric,
    }));

    let scene = scene;
    let camera = Camera::default();

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

    for j in (0..height).rev() {
        for i in 0..width {
            let mut color_samples = Color::BLACK;

            for s in 0..samples {
                let u = (i as Nm
                    + rand::random::<f32>())
                    / (width as Nm);
                let v = (j as Nm
                    + rand::random::<f32>())
                    / (height as Nm);
                let ray: Ray = camera.ray(u, v);

                color_samples += color(&ray, scene, 0);
            }

            {
                let (r, g, b) = color_samples
                    .darken(samples as f32)
                    .web_color();

                writeln!(
                    &mut o,
                    "{:?} {:?} {:?}",
                    r, g, b,
                )?;
            }
        }
    }
    Ok(())
}
