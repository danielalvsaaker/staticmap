#[cfg(feature = "default-tile-fetcher")]
mod default;
mod noop;

#[cfg(feature = "default-tile-fetcher")]
pub use default::DefaultTileFetcher;
pub use noop::NoopTileFetcher;

/// `TileFetcher` is a trait for fetching Open Street Map raster tiles.
/// It provides a method, `fetch`, for retrieving tiles from specified URLs, returning
/// a vector of results containing either the tile data as bytes or an error.
///
/// You can implement their own `TileFetcher` for specific needs, such as adding
/// caching or API request throttling to manage server load effectively.
///
/// # Implementations
/// - `DefaultTileFetcher`: Utilizes `attohttpc` for HTTP requests and `rayon` for
///   parallel fetching. Enabled with default features, it offers efficient and concurrent
///   tile retrieval.
/// - `NoopTileFetcher`: Active when default features are disabled, performs no actual
///   HTTP requests. Instead, it returns a single-pixel PNG image, avoiding dependencies
///   on `attohttpc` and `rayon`.
pub trait TileFetcher {
    /// Fetches raster tiles from specified URLs.
    ///
    /// # Parameters
    /// - `tile_urls`: A slice of string slices, each representing a URL from which to fetch a tile.
    ///
    /// # Returns
    /// A `Vec<Result<Vec<u8>, crate::error::Error>>`, with each element corresponding to a tile fetch attempt.
    /// On success, an element is `Ok(Vec<u8>)` with the tile's bytes. On failure, it is recommended to
    /// wrap the error in `crate::error::Error::TileError` variant, resulting in `Err(crate::error::Error)`
    /// that provides details of the encountered issue and the URL of the failed request.
    fn fetch(&self, tile_urls: &[&str]) -> Vec<std::result::Result<Vec<u8>, crate::error::Error>>;
}
