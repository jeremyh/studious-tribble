#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ray::Ray;
use crate::vec3::Vec3;
use color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::Path;
use vec3::Nm;

mod color;
mod ray;
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

/// Does the ray hit our sphere?
/// If so, return the time t of the hit.
fn hit_sphere(
    center: Vec3,
    radius: f32,
    ray: &Ray,
) -> Option<f32> {
    let oc = ray.origin - center;
    let a = ray.direction.dots();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dots() - radius * radius;
    let discriminant = b * b - 4. * a * c;

    if discriminant > 0. {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    } else {
        None
    }
}

fn color(ray: &Ray) -> Color {
    if let Some(t) =
        hit_sphere(Vec3::new(0., 0., -1.), 0.5, ray)
    {
        let surface_normal = (ray.point_at(t)
            - Vec3::new(0., 0., -1.))
        .unit();

        // Normal was in range -1 to +1
        // Convert to range 0-1 for our colors.
        return Color::from(
            (surface_normal + 1.) * 0.5,
        );
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);

    Color::linear(Color::WHITE, Color::SKY_BLUE, t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("image.ppm");
    let mut o = BufWriter::new(File::create(&path)?);

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = ZERO;

    const SIZE: (i32, i32) = (400, 200);
    let (width, height) = SIZE;

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

            let c = color(&ray);
            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                c.r, c.g, c.b
            )?;
        }
    }

    Ok(())
}
