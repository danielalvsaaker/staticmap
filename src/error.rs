#[derive(Debug)]
pub enum StaticMapError {
    MapError(String),
}

impl std::error::Error for StaticMapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StaticMapError::MapError(_) => None,
        }
    }
}

impl std::fmt::Display for StaticMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            StaticMapError::MapError(ref error) => write!(f, "Map error: {}", error),
        }
    }
}
