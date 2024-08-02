use attohttpc::{Method, RequestBuilder, Response};
use rayon::prelude::*;

use crate::{fetchers::TileFetcher, Error};

/// `DefaultTileFetcher` is the default implementation of the `TileFetcher` trait,
/// designed to fetch raster tiles from Open Street Map (OSM) providers concurrently.
/// It leverages `attohttpc` for making HTTP GET requests and `rayon` for parallelizing
/// the fetch operations across multiple URLs.
///
/// If the request is successful, it returns the raw bytes of the tile image.
/// In case of failure, it wraps the error along with the offending URL in a
/// `TileError` and returns it, allowing for easy identification of problematic requests.
///
/// This implementation is suitable for most use cases requiring tile fetching from
/// OSM or similar tile providers. It optimizes for speed and error handling, making
/// it a robust choice for applications needing reliable tile retrieval.
pub struct DefaultTileFetcher;

impl TileFetcher for DefaultTileFetcher {
    fn fetch(&self, tile_urls: &[&str]) -> Vec<std::result::Result<Vec<u8>, crate::error::Error>> {
        let results = tile_urls
            .par_iter()
            .map(|&tile_url| {
                RequestBuilder::try_new(Method::GET, tile_url)
                    .and_then(RequestBuilder::send)
                    .and_then(Response::bytes)
            })
            .collect::<Vec<_>>();
        results
            .into_iter()
            .zip(tile_urls)
            .map(|(res, &tile_url)| {
                res.map_err(|e| Error::TileError {
                    error: Box::new(e),
                    url: tile_url.to_owned(),
                })
            })
            .collect()
    }
}
