#[derive(Debug)]
pub enum StaticMapError {
    PngEncodingError(png::EncodingError),
    TileError {
        error: png::DecodingError,
        url: String,
    },
    InvalidSize,
}

impl std::error::Error for StaticMapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StaticMapError::PngEncodingError(ref error) => Some(error),
            StaticMapError::TileError { ref error, .. } => Some(error),
            _ => None,
        }
    }
}

impl std::fmt::Display for StaticMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            StaticMapError::InvalidSize => write!(f, "Width or height of map is invalid."),
            StaticMapError::PngEncodingError(ref error) => write!(f, "{}", error),
            StaticMapError::TileError { ref error, ref url } => write!(
                f,
                "{}. Failed to get or encode tile with url {}",
                error, url
            ),
        }
    }
}
