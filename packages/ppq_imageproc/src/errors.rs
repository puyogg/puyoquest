use image::ImageError;

#[derive(Debug)]
pub enum ImageProcError {
    IoCursorError(std::io::Error),
    ImageDecodeError(ImageError),
    PngConversionError(ImageError),
}

impl std::error::Error for ImageProcError {}
impl std::fmt::Display for ImageProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoCursorError(error) => error.fmt(f),
            Self::ImageDecodeError(error) => error.fmt(f),
            Self::PngConversionError(error) => error.fmt(f),
        }
    }
}
