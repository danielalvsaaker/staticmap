#[derive(Debug)]
pub enum StaticMapError {
    ImageError { source: image::error::ImageError },

    MapError(String),
}

impl std::error::Error for StaticMapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StaticMapError::ImageError { ref source } => Some(source),
            StaticMapError::MapError(_) => None,
        }
    }
}

impl std::fmt::Display for StaticMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            StaticMapError::MapError(ref error) => write!(f, "Map error: {}", error),
            StaticMapError::ImageError { ref source } => {
                write!(f, "Image encoding/decoding error: {}", source)
            },
        }
    }
}

impl From<image::error::ImageError> for StaticMapError {
    fn from(err: image::error::ImageError) -> StaticMapError {
        StaticMapError::ImageError { source: err }
    }
}
