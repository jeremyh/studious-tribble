use crate::color::{Color, WebColor};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as IoWrite;

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
    ) -> Result<(), anyhow::Error> {
        let image = self;
        let mut o = BufWriter::new(File::create(path)?);
        writeln!(
            &mut o,
            "P3\n{nx} {ny}\n255",
            nx = image.width(),
            ny = image.height()
        )?;

        image.for_each(move |_, _, color: &Color| {
            let WebColor(r, g, b) = color.web_color();
            writeln!(
            &mut o,
            "{:?} {:?} {:?}",
            r, g, b,
        ).unwrap();
        });

        Ok(())
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
