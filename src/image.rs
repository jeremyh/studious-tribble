use crate::color::{Color, WebColor};
use std::fs::File;
use std::io::Write as IoWrite;
use std::io::{BufWriter, Write};

use byteorder::{ByteOrder, LittleEndian};
use eyre::eyre;

use crate::vec3::{Vec3, F};
use std::{ops::AddAssign, path::Path};

pub struct Image {
    image: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: vec![
                vec![Color::black(); width];
                height
            ],
        }
    }
    pub fn width(&self) -> usize {
        self.image[0].len()
    }
    pub fn height(&self) -> usize {
        self.image.len()
    }
    pub fn add_average(&mut self, others: &[Self]) {
        for (j, row) in
            self.image.iter_mut().enumerate()
        {
            for (i, out_color) in
                row.iter_mut().enumerate()
            {
                for other in others {
                    *out_color += other.image[j][i];
                }
                *out_color = out_color
                    .darken(others.len() as f32);
            }
        }
    }

    pub fn average(images: &mut [Self]) -> Image {
        let len = images.len() as f32;
        let (first, remaining) =
            images.split_first().unwrap();
        let mut image =
            Image::new(first.width(), first.height());
        for (j, row) in
            image.image.iter_mut().enumerate()
        {
            for (i, color) in row.iter_mut().enumerate()
            {
                *color = first.image[j][i];
                for other in remaining {
                    *color += other.image[j][i];
                }
                *color = color.darken(len);
            }
        }
        image
    }

    /// Loop over every pixel, top to bottom, left to right.
    pub fn for_each<TakePixel>(&self, mut f: TakePixel)
    where
        TakePixel: FnMut(usize, usize, &Color),
    {
        for (j, row) in
            self.image.iter().rev().enumerate()
        {
            for (i, out_color) in row.iter().enumerate()
            {
                f(j, i, out_color);
            }
        }
    }

    pub fn write(
        &self,
        path: &Path,
    ) -> color_eyre::Result<()> {
        match path
            .extension()
            .unwrap_or_default()
            .to_str()
            .expect("Non-utf-8 file extension")
        {
            "ppm" => {
                write_ppm_file(
                    self,
                    BufWriter::new(File::create(path)?)
                )
            }
            "tga" => {
                write_tga_file(
                    self,
                    BufWriter::new(File::create(path)?),
                )
            }
            _ => {
                 Err(eyre!(
                "Unsupported output image extension (try. Got {:?}", path.extension().unwrap_or_default()
            ))
            }
        }
    }
}

impl Into<Vec<Vec<Color>>> for Image {
    fn into(self) -> Vec<Vec<Color>> {
        self.image
    }
}
impl Into<Image> for Vec<Vec<Color>> {
    fn into(self) -> Image {
        Image { image: self }
    }
}
impl AddAssign for Image {
    fn add_assign(&mut self, rhs: Self) {
        for (j, row) in
            self.image.iter_mut().enumerate()
        {
            for (i, out_color) in
                row.iter_mut().enumerate()
            {
                *out_color += rhs.image[j][i];
            }
        }
    }
}

/// Write as TARGA file format
fn write_tga_file<O>(
    image: &Image,
    mut out: O,
) -> color_eyre::Result<()>
where
    O: Write,
{
    let mut header = [0u8; 18];

    header[2] = 2;
    LittleEndian::write_u16(
        &mut header[12..14],
        image.width() as u16,
    );
    LittleEndian::write_u16(
        &mut header[14..16],
        image.height() as u16,
    );

    header[16] = 24;
    header[17] = 32;

    out.write_all(&header)?;

    image.for_each(move |_, _, color: &Color| {
        let pixel: [u8; 3] = color.web_color().into();
        out.write_all(&pixel).unwrap();
    });

    Ok(())
}

/// Write as NetPPM file format (text-based)
fn write_ppm_file<O>(
    image: &Image,
    mut o: O,
) -> color_eyre::Result<()>
where
    O: Write,
{
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = image.width(),
        ny = image.height()
    )?;

    image.for_each(move |_, _, color: &Color| {
        let p = color.web_color();
        writeln!(
            &mut o,
            "{:?} {:?} {:?}",
            p.r(),
            p.g(),
            p.b(),
        )
        .unwrap();
    });

    Ok(())
}
