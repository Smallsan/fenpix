use image::ImageError;

#[derive(Debug)]
pub enum FenToImgError {
    ImageError(ImageError),
    FenError(String),
}

impl From<ImageError> for FenToImgError {
    fn from(err: ImageError) -> FenToImgError {
        FenToImgError::ImageError(err)
    }
}

impl From<&str> for FenToImgError {
    fn from(err: &str) -> FenToImgError {
        FenToImgError::FenError(err.to_string())
    }
}
