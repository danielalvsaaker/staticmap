#[derive(Debug)]
/// An enum containing all possible errors when interacting with this library.
pub enum Error {
    /// Error when encoding image to PNG.
    PngEncodingError(png::EncodingError),

    /// Error when decoding PNG from bytes.
    PngDecodingError(png::DecodingError),

    /// Request error when fetching tile from a tile server.
    TileError {
        /// Internal error from the HTTP client.
        error: attohttpc::Error,
        /// The URL which failed.
        url: String,
    },

    /// Invalid image size.
    InvalidSize,

    /// Missing a field/fields when consuming a builder.
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
