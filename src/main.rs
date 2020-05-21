#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use structopt::StructOpt;

use camera::Camera;
use color::{Color, WebColor};
use vec3::F;

use crate::hitable::Hitable;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::time::format_rough_duration;
use crate::vec3::Vec3;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod scene;
mod scenes;
mod time;
mod vec3;

const MAX_DEPTH: i32 = 50;

fn ray_color(
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
            ray_color(&scattered_ray, scene, depth + 1)
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
    width: usize,
    #[structopt(short, long, default_value = "200")]
    height: usize,

    #[structopt(
        parse(from_os_str),
        default_value = "image.ppm"
    )]
    output: PathBuf,

    #[structopt(long, default_value = "16")]
    samples: u16,

    #[structopt(long, default_value = "3")]
    threads: usize,
}

fn main() -> Result<(), anyhow::Error> {
    let opt: Opt = Opt::from_args();

    let scene = scenes::random_scene();
    let aspect = (opt.width as F) / (opt.height as F);
    eprintln!(
        "Creating {}x{} with {} samples per pixel to {:?}",
        opt.width, opt.height, opt.samples, opt.output
    );

    let look_from = Vec3::new(12., 6., 0.51);
    let look_at = Vec3::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.6;
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
        scene,
        camera,
        opt.width,
        opt.height,
        opt.output.as_path(),
        opt.samples,
        opt.threads,
    )
}

fn render(
    scene: Scene,
    camera: Camera,
    width: usize,
    height: usize,
    path: &Path,
    samples: u16,
    threads: usize,
) -> Result<(), anyhow::Error> {
    // TODO: Split out a separate image writer logic

    let start = Instant::now();

    let rays_to_trace = (width as u64)
        * (height as u64)
        * (samples as u64);
    eprintln!("{} rays   ", rays_to_trace);

    let mut children = Vec::with_capacity(threads);
    let samples_per_thread = samples / (threads as u16);

    let camera: Arc<Camera> = Arc::new(camera);
    let scene: Arc<Scene> = Arc::new(scene);

    for i in 0..threads {
        let scene = scene.clone();
        let camera = camera.clone();
        children.push(thread::spawn(move || {
            render_image(
                scene,
                camera,
                width,
                height,
                samples_per_thread,
            )
        }));
    }

    let mut images = Vec::with_capacity(threads);
    for child in children {
        images.push(child.join().unwrap());
    }

    let mut image =
        vec![vec![WebColor::default(); width]; height];
    for (j, row) in image.iter_mut().enumerate() {
        for (i, out_color) in row.iter_mut().enumerate()
        {
            let c: Color =
                images.iter().map(|c| c[j][i]).sum();

            *out_color = c
                .darken(images.len() as f32)
                .web_color();
        }
    }

    write_image(path, &mut image)?;

    eprintln!(
        "\rRendered in {:<30}",
        format_rough_duration(start.elapsed()),
    );
    eprintln!(
        "{} rays/millisecond",
        (rays_to_trace as u128)
            / start.elapsed().as_millis(),
    );
    Ok(())
}

fn write_image(
    path: &Path,
    image: &mut Vec<Vec<WebColor>>,
) -> Result<(), anyhow::Error> {
    let mut o = BufWriter::new(File::create(path)?);
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = image[0].len(),
        ny = image.len()
    )?;

    for row in image.iter().rev() {
        for WebColor(r, g, b) in row.iter() {
            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                r, g, b,
            )?;
        }
    }

    Ok(())
}

fn render_image(
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    width: usize,
    height: usize,
    samples: u16,
) -> Vec<Vec<Color>> {
    let mut image =
        vec![vec![Color::BLACK; width]; height];
    for (j, row) in image.iter_mut().enumerate() {
        for (i, color) in row.iter_mut().enumerate() {
            let mut color_samples = Color::BLACK;

            for s in 0..samples {
                let u = (i as F + rand::random::<F>())
                    / (width as F);
                let v = (j as F + rand::random::<F>())
                    / (height as F);
                let ray: Ray = camera.ray(u, v);

                color_samples +=
                    ray_color(&ray, scene.as_ref(), 0);
            }

            *color = color_samples.darken(samples as F);
        }
    }
    image
}
