use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{fmt, ops::AddAssign, path::Path};

use crate::color::Color;
use crate::vec3::F;
use crate::Res;

pub struct Image {
    image: Vec<Vec<Color>>,
}

#[derive(Debug)]
pub struct UnsupportedFormatError {
    unsupported_extension: String,
}

impl Display for UnsupportedFormatError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "Unsupported image type {:?}. Perhaps you want '.ff'?", &self.unsupported_extension)?;
        Ok(())
    }
}

impl std::error::Error for UnsupportedFormatError {}

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
        let (first, remaining) = images
            .split_first()
            .expect("cannot average zero images");
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
    pub fn for_each_pixel<PixelTaker>(
        &self,
        mut f: PixelTaker,
    ) -> Res<()>
    where
        PixelTaker:
            FnMut(usize, usize, &Color) -> Res<()>,
    {
        for (j, row) in
            self.image.iter().rev().enumerate()
        {
            for (i, out_color) in row.iter().enumerate()
            {
                f(j, i, out_color)?;
            }
        }
        Ok(())
    }

    pub fn write(&self, path: &Path) -> Res<()> {
        match path
            .extension()
            .unwrap_or_default()
            .to_str()
            .expect("Non-utf-8 file extension")
        {
            "ff" => write_farbfeld_file(
                self,
                BufWriter::new(File::create(path)?),
            ),
            "ppm" => write_ppm_file(
                self,
                BufWriter::new(File::create(path)?),
            ),
            "tga" => write_tga_file(
                self,
                BufWriter::new(File::create(path)?),
            ),
            _ => {
                Err(Box::new(UnsupportedFormatError {
                    unsupported_extension: path
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or("<invalid-utf-8>")
                        .to_owned(),
                }))
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
fn write_tga_file(
    image: &Image,
    mut out: impl Write,
) -> Res<()> {
    let mut header = [0u8; 18];
    header[2] = 2;
    header[12..14].clone_from_slice(
        &(image.width() as u16).to_le_bytes(),
    );
    header[14..16].clone_from_slice(
        &(image.height() as u16).to_le_bytes(),
    );
    header[16] = 24;
    header[17] = 32;

    out.write_all(&header)?;
    image.for_each_pixel(
        move |_, _, color: &Color| {
            let pixel: [u8; 3] =
                color.to_web_color().into();
            out.write_all(&pixel)?;
            Ok(())
        },
    )?;

    Ok(())
}

/// Write as farbfeld image format
fn write_farbfeld_file(
    image: &Image,
    mut out: impl Write,
) -> Res<()> {
    out.write_all(b"farbfeld")?;
    out.write_all(
        &(image.width() as u32).to_be_bytes(),
    )?;
    out.write_all(
        &(image.height() as u32).to_be_bytes(),
    )?;

    image.for_each_pixel(
        move |_, _, color: &Color| {
            let color = color.gamma_corrected();
            let mut write_pixel = |f: F| {
                out.write(
                    &((f * (u16::max_value() as F))
                        as u16)
                        .to_be_bytes(),
                )
            };

            write_pixel(color.r())?;
            write_pixel(color.g())?;
            write_pixel(color.b())?;
            // Alpha
            out.write_all(&[255, 255])?;
            Ok(())
        },
    )?;

    Ok(())
}

/// Write as NetPPM file format (text-based)
fn write_ppm_file(
    image: &Image,
    mut o: impl Write,
) -> Res<()> {
    writeln!(
        &mut o,
        "P3\n{nx} {ny}\n255",
        nx = image.width(),
        ny = image.height()
    )?;

    image.for_each_pixel(
        move |_, _, color: &Color| {
            let p = color.to_web_color();
            writeln!(
                &mut o,
                "{:?} {:?} {:?}",
                p.r(),
                p.g(),
                p.b(),
            )?;
            Ok(())
        },
    )?;

    Ok(())
}
