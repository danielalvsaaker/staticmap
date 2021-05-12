#[derive(Debug)]
pub enum Error {
    /// Error when encoding image to png.
    PngEncodingError(png::EncodingError),
    PngDecodingError(png::DecodingError),
    /// Either invalid url or request failure.
    TileError {
        error: attohttpc::Error,
        url: String,
    },
    /// Invalid image size.
    InvalidSize,
    BuildError(&'static str),
}

impl From<png::EncodingError> for Error {
    fn from(e: png::EncodingError) -> Self {
        Self::PngEncodingError(e)
    }
}

impl From<png::DecodingError> for Error {
    fn from(e: png::DecodingError) -> Self {
        Self::PngDecodingError(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::PngEncodingError(ref error) => Some(error),
            Error::PngDecodingError(ref error) => Some(error),
            Error::TileError { ref error, .. } => Some(error),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidSize => write!(f, "Width or height of map is invalid."),
            Error::PngEncodingError(ref error) => write!(f, "{}.", error),
            Error::PngDecodingError(ref error) => write!(f, "{}.", error),
            Error::BuildError(ref error) => write!(f, "{}.", error),
            Error::TileError { ref error, ref url } => {
                write!(
                    f,
                    "Failed to get tile with url {}. Internal error: {}.",
                    url, error
                )
            }
        }
    }
}
