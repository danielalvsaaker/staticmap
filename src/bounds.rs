use crate::{lat_to_y, lon_to_x, tools::Tool};

/// Helper struct for converting to pixels,
/// and to pass information about map bounds to implementors of [Tool][Tool].
pub struct Bounds {
    /// Height of the map in pixels.
    pub height: u32,

    /// Width of the map in pixels.
    pub width: u32,

    /// X coordinate of the map's center.
    pub x_center: f64,

    /// Y coordinate of the map's center.
    pub y_center: f64,

    /// Minimum tile on the x axis.
    pub x_min: i32,

    /// Maximum tile on the x axis.
    pub x_max: i32,

    /// Minimum tile on the y axis.
    pub y_min: i32,

    /// Maximum tile on the y axis.
    pub y_max: i32,

    /// Tile size in pixels.
    pub tile_size: u32,

    /// Map zoom.
    pub zoom: u8,
}

impl Bounds {
    /// Helper function for converting an x coordinate to pixel.
    pub fn x_to_px(&self, x: f64) -> f64 {
        let px = (x - self.x_center) * f64::from(self.tile_size) + f64::from(self.width) / 2.;
        px.round()
    }

    /// Helper function for converting a y coordinate to pixel.
    pub fn y_to_px(&self, y: f64) -> f64 {
        let px = (y - self.y_center) * f64::from(self.tile_size) + f64::from(self.height) / 2.;
        px.round()
    }
}

#[derive(Default)]
/// Builder for [Bounds][Bounds].
pub struct BoundsBuilder {
    lon_min: f64,
    lat_min: f64,
    lon_max: f64,
    lat_max: f64,
    zoom: Option<u8>,
    height: u32,
    width: u32,
    padding: (u32, u32),
    tile_size: u32,
    lat_center: Option<f64>,
    lon_center: Option<f64>,
}

impl BoundsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn zoom(mut self, zoom: Option<u8>) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn tile_size(mut self, size: u32) -> Self {
        self.tile_size = size;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn lon_center(mut self, center: Option<f64>) -> Self {
        self.lon_center = center;
        self
    }

    pub fn lat_center(mut self, center: Option<f64>) -> Self {
        self.lat_center = center;
        self
    }

    pub fn padding(mut self, padding: (u32, u32)) -> Self {
        self.padding = padding;
        self
    }

    pub fn build(&mut self, tools: &[Box<dyn Tool>]) -> Bounds {
        let zoom = if let Some(z) = self.zoom {
            self.determine_extent(z, tools);
            z
        } else {
            self.calculate_zoom(tools)
        };

        let (x_center, y_center) = match self.lon_center.zip(self.lat_center) {
            Some((lon, lat)) => (lon_to_x(lon, zoom), lat_to_y(lat, zoom)),
            _ => {
                let x_min = lon_to_x(self.lon_min, zoom);
                let x_max = lon_to_x(self.lon_max, zoom);
                let y_min = lat_to_y(self.lat_max, zoom);
                let y_max = lat_to_y(self.lat_min, zoom);
                ((x_min + x_max) / 2., (y_min + y_max) / 2.)
            }
        };

        let x_m = 0.5 * f64::from(self.width) / f64::from(self.tile_size);
        let y_m = 0.5 * f64::from(self.height) / f64::from(self.tile_size);

        let x_min = (x_center - x_m).floor() as i32;

        let x_max = (x_center + x_m).ceil() as i32;

        let y_min = (y_center - y_m).floor() as i32;

        let y_max = (y_center + y_m).ceil() as i32;

        Bounds {
            height: self.height,
            width: self.width,
            x_center,
            y_center,
            x_min,
            x_max,
            y_min,
            y_max,
            tile_size: self.tile_size,
            zoom,
        }
    }

    #[inline]
    fn determine_height(&self, zoom: u8) -> f64 {
        (lat_to_y(self.lat_min, zoom) - lat_to_y(self.lat_max, zoom)) * f64::from(self.tile_size)
    }

    #[inline]
    fn determine_width(&self, zoom: u8) -> f64 {
        (lon_to_x(self.lon_max, zoom) - lon_to_x(self.lon_min, zoom)) * f64::from(self.tile_size)
    }

    #[inline]
    fn determine_extent(&mut self, zoom: u8, tools: &[Box<dyn Tool>]) {
        let extent: Vec<(f64, f64, f64, f64)> = tools
            .iter()
            .map(|x| x.extent(zoom, self.tile_size.into()))
            .collect();

        let lon_min = extent.iter().map(|x| x.0).fold(f64::NAN, f64::min);
        let lat_min = extent.iter().map(|x| x.1).fold(f64::NAN, f64::min);
        let lon_max = extent.iter().map(|x| x.2).fold(f64::NAN, f64::max);
        let lat_max = extent.iter().map(|x| x.3).fold(f64::NAN, f64::max);

        if let (Some(lon), Some(lat)) = (self.lon_center, self.lat_center) {
            // Adjust bounds to center on (lon_center, lat_center), expanding as needed
            self.lon_min = lon_min.min(2. * lon - lon_max);
            self.lat_min = lat_min.min(2. * lat - lat_max);
            self.lon_max = lon_max.max(2. * lon - lon_min);
            self.lat_max = lat_max.max(2. * lat - lat_min);
        } else {
            self.lon_min = lon_min;
            self.lat_min = lat_min;
            self.lon_max = lon_max;
            self.lat_max = lat_max;
        }
    }

    fn calculate_zoom(&mut self, tools: &[Box<dyn Tool>]) -> u8 {
        let mut zoom = 1;
        for z in (0..=17).rev() {
            self.determine_extent(z, tools);

            if self.determine_width(z) > (self.width - self.padding.0 * 2).into() {
                continue;
            }

            if self.determine_height(z) > (self.height - self.padding.1 * 2).into() {
                continue;
            }

            zoom = z;
            break;
        }
        zoom
    }
}
