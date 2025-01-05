use image::{GenericImage, ImageBuffer, ImageError, ImageReader, Rgba, RgbaImage};
use std::io::Cursor;

#[derive(Debug)]
pub enum CardRowsError {
    IoCursorError(std::io::Error),
    ImageDecodeError(ImageError),
    PngConversionError(ImageError),
}

impl std::error::Error for CardRowsError {}
impl std::fmt::Display for CardRowsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoCursorError(error) => error.fmt(f),
            Self::ImageDecodeError(error) => error.fmt(f),
            Self::PngConversionError(error) => error.fmt(f),
        }
    }
}

/// Arranges card icons into a grid with the specified number of columns
pub fn card_rows(
    card_width: u32,
    card_height: u32,
    columns: u32,
    icons: Vec<&[u8]>,
) -> Result<Vec<u8>, CardRowsError> {
    let rows = icons.len() as f64 / f64::from(columns);
    let rows = rows.ceil() as u32;

    let width = card_width * columns;
    let height = card_height * rows;

    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

    for (i, image) in icons.iter().enumerate() {
        let image = image.to_vec();
        let reader = ImageReader::new(Cursor::new(image))
            .with_guessed_format()
            .map_err(CardRowsError::IoCursorError)?;
        let image = reader
            .decode()
            .map_err(CardRowsError::ImageDecodeError)?
            .to_rgba8();

        let x = (i as u32) % columns * card_width;
        let y = (i as u32) / columns * card_height;
        canvas.copy_from(&image, x, y).ok();
    }

    let mut png: Vec<u8> = Vec::new();
    canvas
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(CardRowsError::PngConversionError)?;

    Ok(png)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_image() {
        let image = include_bytes!("../sample_images/Img220207.png");
        let mut images: Vec<&[u8]> = Vec::new();
        for _i in 0..12 {
            images.push(image);
        }

        let output = card_rows(192, 192, 4, images).unwrap();
        // output
        //     .save("./sample_output/test_creates_image.png")
        //     .unwrap();
        std::fs::write("arles.png", output).unwrap();
    }
}
