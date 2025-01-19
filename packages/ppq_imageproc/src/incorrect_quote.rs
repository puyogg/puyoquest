use std::{io::Cursor, num::TryFromIntError, sync::LazyLock};

use ab_glyph::{FontRef, PxScale};
use hyphenation::{Language, Load, Standard};
use image::imageops::flip_horizontal_in_place;
use image::{GenericImage, ImageBuffer, ImageError, ImageReader, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use textwrap::{wrap, WordSplitter};

#[derive(Debug)]
pub enum IncorrectQuoteError {
    IoCursorError(std::io::Error),
    ImageDecodeError(ImageError),
    CanvasCopyFrom(ImageError),
    InvalidFont(ab_glyph::InvalidFont),
    UsizeConversion(TryFromIntError),
    PngConversionError(ImageError),
    MissingQuotes,
}

impl std::error::Error for IncorrectQuoteError {}
impl std::fmt::Display for IncorrectQuoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoCursorError(error) => error.fmt(f),
            Self::ImageDecodeError(error) => error.fmt(f),
            Self::CanvasCopyFrom(error) => error.fmt(f),
            Self::InvalidFont(error) => error.fmt(f),
            Self::UsizeConversion(error) => error.fmt(f),
            Self::PngConversionError(error) => error.fmt(f),
            Self::MissingQuotes => write!(f, "Empty list of quotes"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum IconSide {
    LEFT,
    RIGHT,
}

const PADDING_BETWEEN: u32 = 8;
pub fn incorrect_quote(
    quotes: Vec<(&[u8], IconSide, &str)>,
) -> Result<Vec<u8>, IncorrectQuoteError> {
    if quotes.len() == 0 {
        return Err(IncorrectQuoteError::MissingQuotes);
    }
    let quote_count: u32 = quotes
        .len()
        .try_into()
        .map_err(IncorrectQuoteError::UsizeConversion)?;

    let canvas_height: u32 = (ROW_HEIGHT * quote_count) + (PADDING_BETWEEN * (quote_count - 1));
    let canvas_width: u32 = ROW_WIDTH;
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(canvas_width, canvas_height);

    let padded_height = ROW_HEIGHT + PADDING_BETWEEN;
    for (i, quote) in quotes.iter().enumerate() {
        let i: u32 = i.try_into().map_err(IncorrectQuoteError::UsizeConversion)?;
        let (icon, icon_side, text) = quote;
        let y = padded_height * i;

        let quote_image = match icon_side {
            IconSide::LEFT => quote_icon_left(icon, text)?,
            IconSide::RIGHT => quote_icon_right(icon, text)?,
        };

        canvas
            .copy_from(&quote_image, 0, y)
            .map_err(IncorrectQuoteError::CanvasCopyFrom)?;
    }

    let mut png: Vec<u8> = Vec::new();
    canvas
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(IncorrectQuoteError::PngConversionError)?;

    Ok(png)
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
const ROW_WIDTH: u32 = 912;
const ROW_HEIGHT: u32 = 192;
const ICON_WIDTH_HEIGHT: u32 = 192;
static WORD_WRAP_OPTIONS: LazyLock<textwrap::Options> = LazyLock::new(|| {
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    textwrap::Options::new(46).word_splitter(WordSplitter::Hyphenation(dictionary))
});
const MIKADO_MEDIUM_OTF: &'static [u8] = include_bytes!("./fonts/mikado_medium.otf");

const FONT_SIZE: f32 = 36.0;
const MIDDLE_OFFSET: usize = 36;
const LINE_HEIGHT: i32 = 36;
const FIRST_LINE_Y: i32 = 40;
const TEXT_X_ORIENTATION_LEFT: i32 = ICON_WIDTH_HEIGHT as i32 + 88;
const FONT_COLOR: Rgba<u8> = Rgba([0, 146, 213, 255]);
fn quote_icon_left(
    icon: &[u8],
    text: &str,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, IncorrectQuoteError> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(ROW_WIDTH, ROW_HEIGHT);
    let icon = read_png(icon)?;
    let bubble = read_png(&BUBBLE_LEFT)?;

    canvas
        .copy_from(&icon, 0, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;
    canvas
        .copy_from(&bubble, ICON_WIDTH_HEIGHT, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;

    let wrapped_text = wrap(text, &*WORD_WRAP_OPTIONS);
    for (i, line) in wrapped_text.iter().enumerate() {
        let i: i32 = i.try_into().map_err(IncorrectQuoteError::UsizeConversion)?;
        let offset = match wrapped_text.len() {
            1 => MIDDLE_OFFSET,
            2 => MIDDLE_OFFSET / 2,
            _ => 0,
        };
        let offset: i32 = offset
            .try_into()
            .map_err(IncorrectQuoteError::UsizeConversion)?;

        let x = TEXT_X_ORIENTATION_LEFT;
        let y: i32 = FIRST_LINE_Y + offset + (i * LINE_HEIGHT);

        draw_text_mut(
            &mut canvas,
            FONT_COLOR,
            x,
            y,
            PxScale::from(FONT_SIZE),
            &FontRef::try_from_slice(&MIKADO_MEDIUM_OTF)
                .map_err(IncorrectQuoteError::InvalidFont)?,
            line,
        );
    }

    Ok(canvas)
}

const TEXT_X_ORIENTATION_RIGHT: i32 = 64;
fn quote_icon_right(
    icon: &[u8],
    text: &str,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, IncorrectQuoteError> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(ROW_WIDTH, ROW_HEIGHT);
    let icon = read_png(icon)?;
    let mut bubble = read_png(&BUBBLE_LEFT)?;
    flip_horizontal_in_place(&mut bubble);

    canvas
        .copy_from(&icon, ROW_WIDTH - ICON_WIDTH_HEIGHT, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;
    canvas
        .copy_from(&bubble, 0, 0)
        .map_err(IncorrectQuoteError::CanvasCopyFrom)?;

    let wrapped_text = wrap(text, &*WORD_WRAP_OPTIONS);
    for (i, line) in wrapped_text.iter().enumerate() {
        let i: i32 = i.try_into().map_err(IncorrectQuoteError::UsizeConversion)?;
        let offset = match wrapped_text.len() {
            1 => MIDDLE_OFFSET,
            2 => MIDDLE_OFFSET / 2,
            _ => 0,
        };
        let offset: i32 = offset
            .try_into()
            .map_err(IncorrectQuoteError::UsizeConversion)?;

        let x = TEXT_X_ORIENTATION_RIGHT;
        let y: i32 = FIRST_LINE_Y + offset + (i * LINE_HEIGHT);

        draw_text_mut(
            &mut canvas,
            FONT_COLOR,
            x,
            y,
            PxScale::from(FONT_SIZE),
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

    #[test]
    fn creates_quote_icon_right_3_line() {
        let icon = include_bytes!("../sample_images/Img220207.png");
        let text = "Lorem ipsum blah bleh blegh Popping Puzzle Fun Puyo Puyo Tetris 3 Life is Neverending Pain follow me set me free. TRUst Me and We will esCapE fRoM the CITY. ";

        let image = quote_icon_right(icon, text).unwrap();

        let mut png: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();

        std::fs::write("./sample_output/quote_icon_right_3_line.png", png).unwrap();
    }

    #[test]
    fn creates_conversation() {
        let icon = include_bytes!("../sample_images/Img220207.png");
        let icon = icon.as_ref();

        let conversation = vec![
            (icon, IconSide::LEFT, "Popping Puzzle Fun Puyo Puyo Tetris 3 Life is Neverending Pain follow me set me free. TRUst Me and We will esCapE fRoM the CITY."),
            (icon, IconSide::RIGHT, "Gu gu gu (alright then)"),
            (icon, IconSide::RIGHT, "Gu gu (hanging on the edge of tomorrow from the works of yesterday if you beg or if you borrow you may never find your way)"),
            (icon, IconSide::LEFT, "Sure, let's eat curry tonight, Carbuncle! We can invite Ex Tetris to eat with us too!")
        ];

        let iq = incorrect_quote(conversation).unwrap();
        std::fs::write("./sample_output/incorrect_quote.png", iq).unwrap();
    }
}
