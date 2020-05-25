#![allow(dead_code)]
#![allow(unused_variables)]

use color::Color;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use structopt::StructOpt;

use camera::Camera;

use vec3::F;

use crate::hitable::Hitable;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::time::format_rough_duration;
use crate::vec3::Vec3;
use image::Image;

mod camera;
mod color;
mod hitable;
mod image;
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
            Color::black()
        } else if let Scatter::Scattered {
            ray: scattered_ray,
            attenuation: scattered_attenuation,
        } = hit.material.scatter(ray, &hit)
        {
            ray_color(&scattered_ray, scene, depth + 1)
                .attenuate(scattered_attenuation)
        } else {
            Color::black()
        };
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);

    Color::linear(Color::white(), Color::sky_blue(), t)
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
        "Creating {}x{} with {} samples and {} threads to {:?}",
        opt.width, opt.height, opt.samples, opt.threads, opt.output
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

    let mut images: Vec<Image> =
        Vec::with_capacity(children.len());
    for child in children {
        images.push(child.join().unwrap().into());
    }

    Image::average(&mut images).write(path)?;

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

fn render_image(
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    width: usize,
    height: usize,
    samples: u16,
) -> Vec<Vec<Color>> {
    let mut image =
        vec![vec![Color::black(); width]; height];
    for (j, row) in image.iter_mut().enumerate() {
        for (i, color) in row.iter_mut().enumerate() {
            let mut color_samples = Color::black();

            for s in 0..samples {
                let ray: Ray = camera.ray(
                    (i as F + rand::random::<F>())
                        / (width as F),
                    (j as F + rand::random::<F>())
                        / (height as F),
                );

                color_samples +=
                    ray_color(&ray, scene.as_ref(), 0);
            }

            *color = color_samples.darken(samples as F);
        }
    }
    image
}
