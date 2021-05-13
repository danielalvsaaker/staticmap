use crate::{bounds::Bounds, lat_to_y, lon_to_x, tools::Tool, x_to_lon, y_to_lat, Error, Result};
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};

/// Icon tool.
/// Use [IconBuilder][IconBuilder] as an entrypoint.
///
/// ## Example
/// ```rust
/// use staticmap::tools::IconBuilder;
///
/// let icon = IconBuilder::default()
///     .lat_coordinate(6.5)
///     .lon_coordinate(50.5)
///     .x_offset(3.4)
///     .y_offset(10.)
///     .path("icon.png")
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
pub struct Icon {
    lat_coordinate: f64,
    lon_coordinate: f64,
    x_offset: f64,
    y_offset: f64,
    icon: Pixmap,
}

#[derive(Default)]
/// Builder for [Icon][Icon].
pub struct IconBuilder {
    lat_coordinate: Option<f64>,
    lon_coordinate: Option<f64>,
    x_offset: f64,
    y_offset: f64,
    icon: Option<Pixmap>,
}

impl IconBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Default::default()
    }

    /// **Required**.
    /// The center of the icon as a latitude coordinate.
    pub fn lat_coordinate(mut self, coordinate: f64) -> Self {
        self.lat_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// The center of the icon as a longitude coordinate.
    pub fn lon_coordinate(mut self, coordinate: f64) -> Self {
        self.lon_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// X position of the icon in pixels, relative to the left bottom of the map.
    pub fn x_offset(mut self, offset: f64) -> Self {
        self.x_offset = offset;
        self
    }

    /// **Required**.
    /// Y position of the icon in pixels, relative to the left bottom of the map.
    pub fn y_offset(mut self, offset: f64) -> Self {
        self.y_offset = offset;
        self
    }

    /// **Required**.
    /// Path to a 8-bit PNG image file.
    pub fn path<P: AsRef<std::path::Path>>(mut self, path: P) -> Result<Self> {
        self.icon = Some(Pixmap::load_png(path)?);
        Ok(self)
    }

    /// **Required**.
    /// Load an 8-bit PNG image from bytes.
    pub fn data<D: AsRef<[u8]>>(mut self, data: D) -> Result<Self> {
        self.icon = Some(Pixmap::decode_png(data.as_ref())?);
        Ok(self)
    }

    /// Build the tool, consuming the builder.
    /// Return an error if the builder is missing required fields.
    pub fn build(self) -> Result<Icon> {
        Ok(Icon {
            lat_coordinate: self
                .lat_coordinate
                .ok_or(Error::BuildError("Latitude coordinate not supplied."))?,
            lon_coordinate: self
                .lon_coordinate
                .ok_or(Error::BuildError("Longitude coordinate not supplied."))?,
            x_offset: self.x_offset,
            y_offset: self.y_offset,
            icon: self
                .icon
                .ok_or(Error::BuildError("Icon image not supplied."))?,
        })
    }
}

impl Tool for Icon {
    fn extent(&self, zoom: u8, tile_size: f64) -> (f64, f64, f64, f64) {
        let (width, height): (f64, f64) = (self.icon.width().into(), self.icon.height().into());
        let extent = (
            self.x_offset,
            height - self.y_offset,
            width - self.x_offset,
            self.y_offset,
        );

        let x = lon_to_x(self.lon_coordinate, zoom);
        let y = lat_to_y(self.lat_coordinate, zoom);

        let lon_min = x_to_lon(x - extent.0 / tile_size, zoom);
        let lat_min = y_to_lat(y + extent.1 / tile_size, zoom);
        let lon_max = x_to_lon(x + extent.2 / tile_size, zoom);
        let lat_max = y_to_lat(y - extent.3 / tile_size, zoom);

        (lon_min, lat_min, lon_max, lat_max)
    }

    fn draw(&self, bounds: &Bounds, mut pixmap: PixmapMut) {
        let x = bounds.x_to_px(lon_to_x(self.lon_coordinate, bounds.zoom)) - self.x_offset;
        let y = bounds.y_to_px(lat_to_y(self.lat_coordinate, bounds.zoom)) - self.y_offset;

        pixmap.draw_pixmap(
            x as i32,
            y as i32,
            self.icon.as_ref(),
            &PixmapPaint::default(),
            Transform::default(),
            None,
        );
    }
}
