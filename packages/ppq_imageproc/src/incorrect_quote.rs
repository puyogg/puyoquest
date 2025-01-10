use std::{io::Cursor, num::TryFromIntError, sync::LazyLock};

use ab_glyph::{FontRef, PxScale};
use hyphenation::{Language, Load, Standard};
use image::{GenericImage, ImageBuffer, ImageError, ImageReader, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use textwrap::{wrap, WordSplitter};

#[derive(Debug)]
pub enum IncorrectQuoteError {
    IoCursorError(std::io::Error),
    ImageDecodeError(ImageError),
    CanvasCopyFrom(ImageError),
    InvalidFont(ab_glyph::InvalidFont),
    TooLarge(TryFromIntError),
}

impl std::error::Error for IncorrectQuoteError {}
impl std::fmt::Display for IncorrectQuoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoCursorError(error) => error.fmt(f),
            Self::ImageDecodeError(error) => error.fmt(f),
            Self::CanvasCopyFrom(error) => error.fmt(f),
            Self::InvalidFont(error) => error.fmt(f),
            Self::TooLarge(error) => error.fmt(f),
        }
    }
}

fn read_png(image: &[u8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, IncorrectQuoteError> {
    let reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(IncorrectQuoteError::IoCursorError)?;

    let image = reader
        .decode()
        .map_err(IncorrectQuoteError::ImageDecodeError)?
        .to_rgba8();

    Ok(image)
}

const BUBBLE_LEFT: &'static [u8] = include_bytes!("./images/bubble_left_2024_2.png");
static WORD_WRAP_OPTIONS: LazyLock<textwrap::Options> = LazyLock::new(|| {
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    textwrap::Options::new(52).word_splitter(textwrap::WordSplitter::Hyphenation(dictionary))
});
const MIKADO_MEDIUM_OTF: &'static [u8] = include_bytes!("./fonts/mikado_medium.otf");

fn quote_icon_left(
    icon: &[u8],
    text: &str,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, IncorrectQuoteError> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(912, 192);
    let icon = read_png(icon)?;
    let bubble = read_png(&BUBBLE_LEFT)?;

    canvas
        .copy_from(&icon, 0, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;
    canvas
        .copy_from(&bubble, 192, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;

    let wrapped_text = wrap(text, &*WORD_WRAP_OPTIONS);
    for (i, line) in wrapped_text.iter().enumerate() {
        let offset = match wrapped_text.len() {
            1 => 36,
            2 => 18,
            _ => 0,
        };

        let x = 280;
        let y: i32 = (40 + offset + (i * 36))
            .try_into()
            .map_err(IncorrectQuoteError::TooLarge)?;

        draw_text_mut(
            &mut canvas,
            Rgba([0, 146, 213, 255]),
            x,
            y,
            PxScale::from(32.0),
            &FontRef::try_from_slice(&MIKADO_MEDIUM_OTF)
                .map_err(IncorrectQuoteError::InvalidFont)?,
            line,
        );
    }

    Ok(canvas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_quote_icon_left_1_line() {
        let icon = include_bytes!("../sample_images/Img220207.png");
        let text = "Lorem ipsum blah bleh blegh Popping Puzzle Fun";

        let image = quote_icon_left(icon, text).unwrap();

        let mut png: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();

        std::fs::write("./sample_output/quote_icon_left_1_line.png", png).unwrap();
    }
    
    #[test]
    fn creates_quote_icon_left_2_line() {
        let icon = include_bytes!("../sample_images/Img220207.png");
        let text = "Lorem ipsum blah bleh blegh Popping Puzzle Fun Puyo Puyo Tetris 3 Life is Neverending Pain follow me set";

        let image = quote_icon_left(icon, text).unwrap();

        let mut png: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();

        std::fs::write("./sample_output/quote_icon_left_2_line.png", png).unwrap();
    }

    #[test]
    fn creates_quote_icon_left_3_line() {
        let icon = include_bytes!("../sample_images/Img220207.png");
        let text = "Lorem ipsum blah bleh blegh Popping Puzzle Fun Puyo Puyo Tetris 3 Life is Neverending Pain follow me set me free. TRUst Me and We will esCapE fRoM the CITY. ";

        let image = quote_icon_left(icon, text).unwrap();

        let mut png: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();

        std::fs::write("./sample_output/quote_icon_left_3_line.png", png).unwrap();
    }
}
