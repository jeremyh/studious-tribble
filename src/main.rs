use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::Path;

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
            let r = i as f32 / width as f32;
            let g = j as f32 / height as f32;
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                ir, ig, ib
            )?;
        }
    }

    Ok(())
}
