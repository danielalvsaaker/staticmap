#[cfg(feature = "default-tile-fetcher")]
mod default;
mod noop;

#[cfg(feature = "default-tile-fetcher")]
pub use default::DefaultTileFetcher;
pub use noop::NoopTileFetcher;

pub trait TileFetcher {
    fn fetch(&self, tile_urls: &[&str]) -> Vec<std::result::Result<Vec<u8>, crate::error::Error>>;
}
