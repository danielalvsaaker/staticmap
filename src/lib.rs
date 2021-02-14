use derive_builder::Builder;
use rayon::prelude::*;
use std::f64::consts::PI;
use tiny_skia::*;

mod error;
pub mod tools;

use crate::tools::*;
pub use error::StaticMapError;

type Result<T> = std::result::Result<T, StaticMapError>;

fn default_url_template() -> String {
    "https://a.tile.osm.org/{z}/{x}/{y}.png".to_string()
}

#[derive(Builder)]
/// Main type. Use [StaticMapBuilder][StaticMapBuilder] as an entrypoint.
pub struct StaticMap {
    #[builder(default = "300")]
    /// Image width in pixels. Default is 300.
    pub(crate) width: u32,
    #[builder(default = "300")]
    /// Image height in pixels. Default is 300.
    pub(crate) height: u32,
    #[builder(default)]
    /// Minimal distance between map features and map border in pixels. Given as a tuple of (x, y).
    /// Default is (0, 0).
    pub(crate) padding: (u32, u32),
    #[builder(setter(skip))]
    pub(crate) x_center: f64,
    #[builder(setter(skip))]
    pub(crate) y_center: f64,
    #[builder(setter(strip_option), default)]
    /// Map zoom, usually between 1-17. Calculated based on features if not specified.
    pub(crate) zoom: Option<u8>,
    #[builder(default, setter(strip_option))]
    /// Latitude center of map. Calculated based on features if not specified.
    pub(crate) lat_center: Option<f64>,
    #[builder(default, setter(strip_option))]
    /// Longitude center of map. Calculated based on features if not specified.
    pub(crate) lon_center: Option<f64>,
    #[builder(default = "default_url_template()", setter(into))]
    /// URL template. e.g. "https://a.tile.osm.org/{z}/{x}/{y}.png".
    ///
    /// Default is used if not specified.
    pub(crate) url_template: String,
    #[builder(default = "256")]
    ///Tile size. Default is 256.
    pub(crate) tile_size: u32,
    #[builder(setter(skip))]
    lines: Vec<Line>,
    #[builder(setter(skip))]
    markers: Vec<Box<dyn Marker>>,
    #[builder(setter(skip))]
    extent: (f64, f64, f64, f64),
}

impl StaticMap {
    /// Add a [Line][Line] instance. The map can contain several lines.
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    /// Add a type implementing [Marker][Marker]. The map can contain several markers.
    pub fn add_marker(&mut self, marker: impl Marker + 'static) {
        self.markers.push(Box::new(marker));
    }

    /// Render the map and encode as png.
    ///
    /// May panic if any feature has invalid bounds.
    pub fn encode_png(&mut self) -> Result<Vec<u8>> {
        self.render()?
            .encode_png()
            .map_err(StaticMapError::PngEncodingError)
    }

    /// Render the map and save as png to a file.
    ///
    /// May panic if any feature has invalid bounds.
    pub fn save_png<P: AsRef<::std::path::Path>>(&mut self, path: P) -> Result<()> {
        self.render()?
            .save_png(path)
            .map_err(StaticMapError::PngEncodingError)
    }

    fn render(&mut self) -> Result<Pixmap> {
        if self.zoom.is_none() {
            self.zoom = Some(self.calculate_zoom());
        }

        self.extent = self.determine_extent(self.zoom.unwrap());

        let (lon_center, lat_center) =
            if let (Some(x), Some(y)) = (self.lon_center, self.lat_center) {
                (x, y)
            } else {
                (
                    (self.extent.0 + self.extent.2) / 2.0,
                    (self.extent.1 + self.extent.3) / 2.0,
                )
            };

        self.x_center = lon_to_x(lon_center, self.zoom.unwrap());
        self.y_center = lat_to_y(lat_center, self.zoom.unwrap());

        let mut image = Pixmap::new(self.width, self.height).ok_or(StaticMapError::InvalidSize)?;

        self.draw_base_layer(&mut image)?;

        if !self.markers.is_empty() || !self.lines.is_empty() {
            image.draw_pixmap(
                0,
                0,
                self.draw_features()?.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                None,
            );
        }

        Ok(image)
    }

    fn determine_extent(&self, zoom: u8) -> (f64, f64, f64, f64) {
        let lines = self.lines.iter().map(Tool::extent);

        let markers = self
            .markers
            .iter()
            .map(|m| {
                (
                    lon_to_x(m.lon_coordinate(), zoom),
                    lat_to_y(m.lat_coordinate(), zoom),
                    m.extent(),
                )
            })
            .map(|(x, y, e)| {
                (
                    x_to_lon(x - e.0 / self.tile_size as f64, zoom),
                    y_to_lat(y + e.1 / self.tile_size as f64, zoom),
                    x_to_lon(x + e.2 / self.tile_size as f64, zoom),
                    y_to_lat(y - e.3 / self.tile_size as f64, zoom),
                )
            });

        let extent: Vec<(f64, f64, f64, f64)> = lines.chain(markers).collect();

        let lon_min: f64 = extent.iter().map(|x| x.0).fold(f64::NAN, f64::min);
        let lat_min: f64 = extent.iter().map(|x| x.1).fold(f64::NAN, f64::min);
        let lon_max: f64 = extent.iter().map(|x| x.2).fold(f64::NAN, f64::max);
        let lat_max: f64 = extent.iter().map(|x| x.3).fold(f64::NAN, f64::max);

        (lon_min, lat_min, lon_max, lat_max)
    }

    fn calculate_zoom(&self) -> u8 {
        let mut zoom: u8 = 1;

        let height = |i, (_, x, _, y)| (lat_to_y(x, i) - lat_to_y(y, i)) * self.tile_size as f64;
        let width = |i, (x, _, y, _)| (lon_to_x(y, i) - lon_to_x(x, i)) * self.tile_size as f64;

        for z in (0..=17).rev() {
            let extent = self.determine_extent(z);

            if width(z, extent) > (self.width - self.padding.0 * 2) as f64 {
                continue;
            }

            if height(z, extent) > (self.height - self.padding.1 * 2) as f64 {
                continue;
            }

            zoom = z;
            break;
        }
        zoom
    }

    fn x_to_px(&self, x: f64) -> f64 {
        let px = (x - self.x_center) * self.tile_size as f64 + self.width as f64 / 2f64;
        px.round()
    }

    fn y_to_px(&self, y: f64) -> f64 {
        let px = (y - self.y_center) * self.tile_size as f64 + self.height as f64 / 2f64;
        px.round()
    }

    fn draw_base_layer(&self, image: &mut Pixmap) -> Result<()> {
        let x_min =
            (self.x_center - (0.5 * self.width as f64 / self.tile_size as f64)).floor() as i32;
        let y_min =
            (self.y_center - (0.5 * self.height as f64 / self.tile_size as f64)).floor() as i32;
        let x_max =
            (self.x_center + (0.5 * self.width as f64 / self.tile_size as f64)).ceil() as i32;
        let y_max =
            (self.y_center + (0.5 * self.height as f64 / self.tile_size as f64)).ceil() as i32;

        let mut tiles: Vec<(i32, i32, String)> = Vec::new();
        for x in x_min..x_max {
            for y in y_min..y_max {
                let max_tile: i32 = 2i32.pow(self.zoom.unwrap() as u32);
                let tile_x: i32 = (x + max_tile) % max_tile;
                let tile_y: i32 = (y + max_tile) % max_tile;

                let url = self
                    .url_template
                    .replace("{z}", &self.zoom.unwrap().to_string())
                    .replace("{x}", &tile_x.to_string())
                    .replace("{y}", &tile_y.to_string());
                tiles.push((x, y, url));
            }
        }

        let client = attohttpc::Session::new();
        let tile_images: Vec<Vec<u8>> = tiles
            .par_iter()
            .flat_map(|x| {
                client
                    .get(&x.2)
                    .send()
                    .expect("Failed to send tile request")
                    .bytes()
            })
            .collect();

        for (tile, tile_image) in tiles.iter().zip(tile_images) {
            let (x, y) = (tile.0, tile.1);
            let (x_px, y_px) = (self.x_to_px(x.into()), self.y_to_px(y.into()));

            let pixmap =
                Pixmap::decode_png(&tile_image).map_err(|e| StaticMapError::TileError {
                    error: e,
                    url: tile.2.clone(),
                })?;

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

    fn draw_features(&self) -> Result<Pixmap> {
        let mut pixmap = Pixmap::new(self.width, self.height).ok_or(StaticMapError::InvalidSize)?;

        for line in self.lines.iter() {
            line.draw(self, &mut pixmap);
        }

        for marker in self.markers.iter() {
            marker.draw(self, &mut pixmap);
        }

        Ok(pixmap)
    }
}

fn lon_to_x(lon: f64, zoom: u8) -> f64 {
    let mut lon = lon;

    if !(-180_f64..180_f64).contains(&lon) {
        lon = (lon + 180_f64) % 360_f64 - 180_f64;
    }

    ((lon + 180_f64) / 360_f64) * 2_f64.powi(zoom.into())
}

fn lat_to_y(lat: f64, zoom: u8) -> f64 {
    let mut lat = lat;

    if !(-90_f64..90_f64).contains(&lat) {
        lat = (lat + 90_f64) % 180_f64 - 90_f64;
    }

    (1_f64 - ((lat * PI / 180_f64).tan() + 1_f64 / (lat * PI / 180_f64).cos()).ln() / PI) / 2_f64
        * 2_f64.powi(zoom.into())
}

fn y_to_lat(y: f64, zoom: u8) -> f64 {
    (PI * (1f64 - 2f64 * y / 2f64.powi(zoom.into())))
        .sinh()
        .atan()
        / PI
        * 180f64
}

fn x_to_lon(x: f64, zoom: u8) -> f64 {
    x / 2f64.powi(zoom.into()) * 360f64 - 180f64
}

fn simplify(points: Vec<(f64, f64)>, tolerance: u8) -> Vec<(f64, f64)> {
    if points.is_empty() {
        return points;
    }

    let (new_coordinates, points) = points.split_at(1);
    let mut new_coordinates = new_coordinates.to_vec();

    for point in points {
        let a = new_coordinates.last().unwrap();
        let x = ((a.0 - point.0).powi(2) + (a.1 - point.1).powi(2)).sqrt();

        if x > tolerance as f64 {
            new_coordinates.push(*point)
        }
    }

    new_coordinates.push(*points.last().unwrap());

    new_coordinates
}
