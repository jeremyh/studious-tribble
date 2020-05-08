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

fn color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit();
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

    let (width, height) = (200, 100);
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
