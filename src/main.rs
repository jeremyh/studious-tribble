use color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::Path;

mod color;
mod vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("image.ppm");
    let mut o = BufWriter::new(File::create(&path)?);

    let (width, height) = (200, 100);
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = width,
        ny = height
    )?;

    for j in (0..height).rev() {
        for i in 0..width {
            let c = Color::from_ratio(
                i as f32 / width as f32,
                j as f32 / height as f32,
                0.2,
            );

            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                c.r, c.g, c.b
            )?;
        }
    }

    Ok(())
}
