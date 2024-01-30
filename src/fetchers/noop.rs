use crate::fetchers::TileFetcher;

pub struct NoopTileFetcher;

impl TileFetcher for NoopTileFetcher {
    fn fetch(&self, tile_urls: &[&str]) -> Vec<std::result::Result<Vec<u8>, crate::error::Error>> {
        let one_pixel = &[
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1,
            8, 4, 0, 0, 0, 181, 28, 12, 2, 0, 0, 0, 11, 73, 68, 65, 84, 120, 218, 99, 100, 96, 0,
            0, 0, 6, 0, 2, 48, 129, 208, 47, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ];
        println!("{:?}", one_pixel);
        tile_urls.iter().map(|_| Ok(one_pixel.to_vec())).collect()
    }
}
