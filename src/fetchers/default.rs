use attohttpc::{Method, RequestBuilder, Response};
use rayon::prelude::*;

use crate::{fetchers::TileFetcher, Error};

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
