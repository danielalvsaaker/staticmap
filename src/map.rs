use crate::{
    bounds::{Bounds, BoundsBuilder},
    tools::Tool,
    Error, Result,
};
use attohttpc::{Method, RequestBuilder, Response};
use rayon::prelude::*;
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};

/// Main type.
/// Use [StaticMapBuilder][StaticMapBuilder] as an entrypoint.
///
/// ## Example
/// ```rust
/// use staticmap::StaticMapBuilder;
///
/// let mut map = StaticMapBuilder::new()
///     .width(300)
///     .height(300)
///     .zoom(4)
///     .lat_center(52.6)
///     .lon_center(13.4)
///     .build()
///     .unwrap();
///
/// ```
pub struct StaticMap {
    url_template: String,
    tools: Vec<Box<dyn Tool>>,
    bounds: BoundsBuilder,
}

/// Builder for [StaticMap][StaticMap].
pub struct StaticMapBuilder {
    width: u32,
    height: u32,
    padding: (u32, u32),
    zoom: Option<u8>,
    lat_center: Option<f64>,
    lon_center: Option<f64>,
    url_template: String,
    tile_size: u32,
}

impl Default for StaticMapBuilder {
    fn default() -> Self {
        Self {
            width: 300,
            height: 300,
            padding: (0, 0),
            zoom: None,
            lat_center: None,
            lon_center: None,
            url_template: "https://a.tile.osm.org/{z}/{x}/{y}.png".to_string(),
            tile_size: 256,
        }
    }
}

impl StaticMapBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Default::default()
    }

    /// Image width, in pixels.
    /// Default is 300.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Image height, in pixels.
    /// Default is 300.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Padding between map features and edge of map in x and y direction.
    /// Default is (0, 0).
    pub fn padding(mut self, padding: (u32, u32)) -> Self {
        self.padding = padding;
        self
    }

    /// Map zoom, usually between 1-17.
    /// Determined based on map features if not specified.
    pub fn zoom(mut self, zoom: u8) -> Self {
        self.zoom = Some(zoom);
        self
    }

    /// Latitude center of the map.
    /// Determined based on map features if not specified.
    pub fn lat_center(mut self, coordinate: f64) -> Self {
        self.lat_center = Some(coordinate);
        self
    }

    /// Longitude center of the map.
    /// Determined based on map features if not specified.
    pub fn lon_center(mut self, coordinate: f64) -> Self {
        self.lon_center = Some(coordinate);
        self
    }

    /// URL template, e.g. "https://example.com/{z}/{x}/{y}.png".
    /// Default is "https://a.tile.osm.org/{z}/{x}/{y}.png".
    pub fn url_template<I: Into<String>>(mut self, url_template: I) -> Self {
        self.url_template = url_template.into();
        self
    }

    /// Tile size, in pixels.
    /// Default is 256.
    pub fn tile_size(mut self, tile_size: u32) -> Self {
        self.tile_size = tile_size;
        self
    }

    /// Consumes the builder.
    pub fn build(self) -> Result<StaticMap> {
        let bounds = BoundsBuilder::new()
            .zoom(self.zoom)
            .tile_size(self.tile_size)
            .lon_center(self.lon_center)
            .lat_center(self.lat_center)
            .padding(self.padding)
            .height(self.height)
            .width(self.width);

        Ok(StaticMap {
            url_template: self.url_template,
            tools: Vec::new(),
            bounds,
        })
    }
}

impl StaticMap {
    /// Add a type implementing [Tool][Tool]. The map can contain several tools.
    pub fn add_tool(&mut self, tool: impl Tool + 'static) {
        self.tools.push(Box::new(tool));
    }

    /// Render the map and encode as PNG.
    ///
    /// May panic if any feature has invalid bounds.
    pub fn encode_png(&mut self) -> Result<Vec<u8>> {
        Ok(self.render()?.encode_png()?)
    }

    /// Render the map and save as PNG to a file.
    ///
    /// May panic if any feature has invalid bounds.
    pub fn save_png<P: AsRef<::std::path::Path>>(&mut self, path: P) -> Result<()> {
        self.render()?.save_png(path)?;
        Ok(())
    }

    fn render(&mut self) -> Result<Pixmap> {
        let bounds = self.bounds.build(&self.tools);

        let mut image = Pixmap::new(bounds.width, bounds.height).ok_or(Error::InvalidSize)?;

        self.draw_base_layer(image.as_mut(), &bounds)?;

        for tool in self.tools.iter() {
            tool.draw(&bounds, image.as_mut());
        }

        Ok(image)
    }

    fn draw_base_layer(&self, mut image: PixmapMut, bounds: &Bounds) -> Result<()> {
        let x_min = (bounds.x_center
            - (0.5 * f64::from(bounds.width) / f64::from(bounds.tile_size)))
        .floor() as i32;
        let y_min = (bounds.y_center
            - (0.5 * f64::from(bounds.height) / f64::from(bounds.tile_size)))
        .floor() as i32;
        let x_max = (bounds.x_center
            + (0.5 * f64::from(bounds.width) / f64::from(bounds.tile_size)))
        .ceil() as i32;
        let y_max = (bounds.y_center
            + (0.5 * f64::from(bounds.height) / f64::from(bounds.tile_size)))
        .ceil() as i32;

        let mut tiles: Vec<(i32, i32, String)> = Vec::new();
        for x in x_min..x_max {
            for y in y_min..y_max {
                let max_tile: i32 = 2_i32.pow(bounds.zoom.into());
                let tile_x = (x + max_tile) % max_tile;
                let tile_y = (y + max_tile) % max_tile;

                let url = self
                    .url_template
                    .replace("{z}", &bounds.zoom.to_string())
                    .replace("{x}", &tile_x.to_string())
                    .replace("{y}", &tile_y.to_string());

                tiles.push((x, y, url));
            }
        }

        let tile_images: Vec<_> = tiles
            .par_iter()
            .map(|x| {
                RequestBuilder::try_new(Method::GET, &x.2)
                    .and_then(RequestBuilder::send)
                    .and_then(Response::bytes)
                    .map_err(|error| Error::TileError {
                        error,
                        url: x.2.clone(),
                    })
            })
            .collect();

        for (tile, tile_image) in tiles.iter().zip(tile_images) {
            let (x, y) = (tile.0, tile.1);
            let (x_px, y_px) = (bounds.x_to_px(x.into()), bounds.y_to_px(y.into()));

            let pixmap = Pixmap::decode_png(&tile_image?)?;

            image.draw_pixmap(
                x_px as i32,
                y_px as i32,
                pixmap.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                None,
            );
        }

        Ok(())
    }
}
