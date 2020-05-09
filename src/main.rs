#![allow(dead_code)]
#![allow(unused_variables)]

use crate::hitable::{Hitable, Sphere};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::Path;
use vec3::Nm;

mod color;
mod hitable;
mod ray;
mod scene;
mod vec3;

const ZERO: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 0.,
};
const ONE: Vec3 = Vec3 {
    x: 1.,
    y: 1.,
    z: 1.,
};

fn color(ray: &Ray, scene: &dyn Hitable) -> Color {
    if let Some(h) =
        scene.hit(&ray, &(0f32..99999999f32))
    {
        // Normal was in range -1 to +1
        // Convert to range 0-1 for our colors.
        return Color::from((h.normal + 1.) * 0.5);
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);

    Color::linear(Color::WHITE, Color::SKY_BLUE, t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = ZERO;

    const SIZE: (i32, i32) = (400, 200);
    let (width, height) = SIZE;

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

    // Render
    let path = Path::new("image.ppm");
    let mut o = BufWriter::new(File::create(&path)?);
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = width,
        ny = height
    )?;
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

            let c = color(&ray, &scene);
            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                c.r, c.g, c.b
            )?;
        }
    }

    Ok(())
}
