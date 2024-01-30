use attohttpc::{Method, RequestBuilder, Response};
use rayon::prelude::*;

use crate::{fetchers::TileFetcher, Error};

pub struct DefaultTileFetcher;

impl TileFetcher for DefaultTileFetcher {
    fn fetch(&self, tile_urls: &[&str]) -> Vec<std::result::Result<Vec<u8>, crate::error::Error>> {
        tile_urls
            .par_iter()
            .map(|tile_url| {
                RequestBuilder::try_new(Method::GET, &tile_url)
                    .and_then(RequestBuilder::send)
                    .and_then(Response::bytes)
                    .map_err(|error| Error::TileError {
                        error: Box::new(error),
                        url: tile_url.to_string(),
                    })
            })
            .collect()
    }
}
