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

mod color;
mod hitable;
mod ray;
mod scene;
mod vec3;

fn color(ray: &Ray, scene: &dyn Hitable) -> Color {
    if let Some(h) =
        scene.hit(&ray, &(0f32..f32::INFINITY))
    {
        // Normal was in range -1 to +1
        // Convert to range 0-1 for our colors.
        return Color::from((h.normal + 1.) * 0.5);
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

    render(
        &scene,
        opt.width,
        opt.height,
        opt.output.as_path(),
    )
}

fn render(
    scene: &Scene,
    width: i32,
    height: i32,
    path: &Path,
) -> Result<(), anyhow::Error> {
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::ZERO;

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
            let hp = (i as Nm) / (width as Nm);
            let vp = (j as Nm) / (height as Nm);
            let ray: Ray = Ray::new(
                origin,
                lower_left_corner
                    + horizontal * hp
                    + vertical * vp,
            );

            let c = color(&ray, scene);
            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                c.r, c.g, c.b
            )?;
        }
    }
    Ok(())
}
