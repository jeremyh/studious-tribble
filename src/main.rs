#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Rem;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use structopt::StructOpt;

use camera::Camera;
use color::Color;
use image::Image;
use vec3::F;

use crate::hitable::Hitable;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::time::format_rough_duration;
use crate::vec3::{randf, Vec3};
use std::io::Write;

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

type Error = Box<dyn std::error::Error>;
type Res<T> = Result<T, Error>;

const MAX_DEPTH: i32 = 50;
const CLEAR_LINE: &str = "\x1b[2K";

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
        default_value = "image.ff"
    )]
    output: PathBuf,

    #[structopt(long, default_value = "16")]
    samples: u16,

    #[structopt(long, default_value = "3")]
    threads: usize,
}

fn main() -> Res<()> {
    let opt: Opt = Opt::from_args();

    let scene = scenes::random_scene();
    let aspect = (opt.width as F) / (opt.height as F);
    eprintln!(
        "{}x{}, {} samples, {} threads.\nOutput: \"{}\"",
        opt.width,
        opt.height,
        opt.samples,
        opt.threads,
        bold(
            opt.output
                .to_str()
                .unwrap_or("<non-utf-8>")
        )
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

/// Percentage this thread has completed so far.
struct ProcStatus {
    thread_id: usize,
    fraction_complete: f32,
}

fn render(
    scene: Scene,
    camera: Camera,
    width: usize,
    height: usize,
    path: &Path,
    samples: u16,
    thread_count: usize,
) -> Res<()> {
    let start = Instant::now();

    let rays_to_trace = (width as u64)
        * (height as u64)
        * (samples as u64);

    let mut children = Vec::with_capacity(thread_count);
    let samples_per_thread =
        samples / (thread_count as u16);

    let camera: Arc<Camera> = Arc::new(camera);
    let scene: Arc<Scene> = Arc::new(scene);

    let (tx, rx) = mpsc::channel::<ProcStatus>();

    for thread_id in 0..thread_count {
        let scene = scene.clone();
        let camera = camera.clone();
        let tx = tx.clone();
        children.push(thread::spawn(move || {
            render_image(
                scene,
                camera,
                width,
                height,
                samples_per_thread,
                |fraction_complete| {
                    tx.send(ProcStatus {
                        thread_id,
                        fraction_complete,
                    })
                        .unwrap_or_else(|e| {
                            eprintln!(
                                "Cannot report processing status from thread {}: {:?}",
                                thread_id, e
                            )
                        })
                },
            )
        }));
    }

    track_thread_progress(thread_count, rx)?;

    let mut images: Vec<Image> =
        Vec::with_capacity(children.len());
    for child in children {
        images.push(
            child
                .join()
                .expect("Failed thread: cannot join")
                .into(),
        );
    }

    Image::average(&mut images).write(path)?;

    eprintln!(
        "\r{} rays, rendered in {:<30}",
        rays_to_trace,
        bold(&format_rough_duration(start.elapsed())),
    );
    eprintln!(
        "{} rays/millisecond",
        (rays_to_trace as u128)
            / start.elapsed().as_millis(),
    );
    Ok(())
}

fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[m", text)
}

fn track_thread_progress(
    threads: usize,
    rx: Receiver<ProcStatus>,
) -> Res<()> {
    let mut statuses = vec![0.; threads];

    loop {
        // Drain any waiting thread statuses.
        for ProcStatus {
            thread_id,
            fraction_complete,
        } in rx.try_iter()
        {
            statuses[thread_id] = fraction_complete;
        }

        // Print statuses. If all are completed, we can exit.
        let mut are_finished = true;
        {
            let out = std::io::stdout();
            let mut out = out.lock();

            writeln!(out)?;
            for (thread_num, fraction_complete) in
                statuses.iter().enumerate()
            {
                write!(
                    out,
                    "{} {:2>} ",
                    CLEAR_LINE, thread_num,
                )?;
                print_progress_bar(
                    *fraction_complete,
                    &mut out,
                )?;
                writeln!(
                    out,
                    " {:2>2.0}% ",
                    *fraction_complete * 100.
                )?;

                if *fraction_complete < 1. {
                    are_finished = false;
                }
            }
            writeln!(out)?;

            if are_finished {
                break Ok(());
            }
            // We're redrawing. Move cursor back up the same number of lines
            write!(
                out,
                "\x1b[{}A",
                // A line (progress bar) per thread, plus header and footer line.
                threads + 2
            )?;
        }

        // Delay before rerender
        thread::sleep(Duration::from_millis(300));
    }
}

fn print_progress_bar(
    fraction_complete: f32,
    out: &mut impl Write,
) -> Res<()> {
    const PROGRESS_BAR_WIDTH: i16 = 40;
    let asdf = "m▐▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░▌  56";
    write!(out, "▐")?;
    let columns_complete = (fraction_complete
        * (PROGRESS_BAR_WIDTH as f32))
        as i16;
    for i in 0..(columns_complete) {
        write!(out, "▓")?;
    }
    for i in columns_complete..(PROGRESS_BAR_WIDTH) {
        write!(out, "░")?;
    }
    write!(out, "▌")?;
    Ok(())
}

fn render_image(
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    width: usize,
    height: usize,
    samples: u16,
    send_fraction_complete_status: impl Fn(f32),
) -> Vec<Vec<Color>> {
    let mut image =
        vec![vec![Color::black(); width]; height];

    // Send status every x rows.
    let every_x = height / 200 + 1;

    for (j, row) in image.iter_mut().enumerate() {
        if (j.rem(every_x)) == 0 {
            send_fraction_complete_status(
                (j as f32) / (height as f32),
            );
        }
        for (i, color) in row.iter_mut().enumerate() {
            let mut color_samples = Color::black();

            for s in 0..samples {
                let ray: Ray = camera.ray(
                    (i as F + randf()) / (width as F),
                    (j as F + randf()) / (height as F),
                );

                color_samples +=
                    ray_color(&ray, scene.as_ref(), 0);
            }

            *color = color_samples.darken(samples as F);
        }
    }

    // We're done!
    send_fraction_complete_status(1.);

    image
}
