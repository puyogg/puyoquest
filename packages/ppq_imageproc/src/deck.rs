use image::{GenericImage, ImageBuffer, ImageReader, Rgba, RgbaImage, imageops::FilterType};
use std::io::Cursor;

use crate::ImageProcError;

const DECK_FRAME: &'static [u8] = include_bytes!("./images/deck_with_supporter_base.png");

pub fn deck(icons: Vec<&[u8]>, support: Option<&[u8]>) -> Result<Vec<u8>, ImageProcError> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(790, 274);

    for (i, image) in icons.iter().enumerate() {
        let image = image.to_vec();
        let reader = ImageReader::new(Cursor::new(image))
            .with_guessed_format()
            .map_err(ImageProcError::IoCursorError)?;
        let image = reader
            .decode()
            .map_err(ImageProcError::ImageDecodeError)?
            .to_rgba8();

        if i == 0 {
            let image = image::imageops::resize(&image, 128, 128, FilterType::Lanczos3);
            canvas.copy_from(&image, 50, 14).ok();
        } else if i >= 1 && i <= 4 {
            let image = image::imageops::resize(&image, 96, 96, FilterType::Lanczos3);
            let x = (188 + (i - 1) * 106) as u32;
            canvas.copy_from(&image, x, 14).ok();
        } else if i >= 5 && i <= 8 {
            let image = image::imageops::resize(&image, 96, 96, FilterType::Lanczos3);
            let x = (188 + (i - 5) * 106) as u32;
            canvas.copy_from(&image, x, 139).ok();
        };
    }

    if let Some(support) = support {
        let image = support.to_vec();
        let reader = ImageReader::new(Cursor::new(image))
            .with_guessed_format()
            .map_err(ImageProcError::IoCursorError)?;
        let image = reader
            .decode()
            .map_err(ImageProcError::ImageDecodeError)?
            .to_rgba8();
        let image = image::imageops::resize(&image, 128, 128, FilterType::Lanczos3);
        canvas.copy_from(&image, 612, 14).ok();
    }

    let deck_frame = read_png(&DECK_FRAME)?;
    image::imageops::overlay(&mut canvas, &deck_frame, 0, 0);

    let mut png: Vec<u8> = Vec::new();
    canvas
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(ImageProcError::PngConversionError)?;

    Ok(png)
}

pub fn read_png(image: &[u8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageProcError> {
    let reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(ImageProcError::IoCursorError)?;

    let image = reader
        .decode()
        .map_err(ImageProcError::ImageDecodeError)?
        .to_rgba8();

    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_image() {
        let image = include_bytes!("../sample_images/Img220207.png");
        let mut icons: Vec<&[u8]> = Vec::new();
        for _i in 0..9 {
            icons.push(image);
        }

        let supporter = image.clone();

        let output = deck(icons, Some(&supporter)).unwrap();
        std::fs::write("./sample_output/test_deck.png", output).unwrap();
    }
}
