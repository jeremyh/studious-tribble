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
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use camera::Camera;

mod camera;
mod color;
mod hitable;
mod ray;
mod scene;
mod vec3;

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::random() * 2.0 - Vec3::ONE;

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn color(ray: &Ray, scene: &dyn Hitable) -> Color {
    if let Some(h) =
        scene.hit(&ray, &(0.001f32..f32::INFINITY))
    {
        let target: Vec3 =
            h.p + h.normal + random_in_unit_sphere();
        // Normal was in range -1 to +1
        // Convert to range 0-1 for our colors.
        return color(
            &Ray::new(h.p, target - h.p),
            scene,
        )
        .darken(2.);
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
    scene.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
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

                color_samples += color(&ray, scene);
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
