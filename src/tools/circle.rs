use crate::{
    bounds::Bounds,
    lat_to_y, lon_to_x, m_to_px,
    tools::{Color, Tool},
    x_to_lon, y_to_lat, Error, Result,
};
use tiny_skia::{FillRule, PathBuilder, PixmapMut, Stroke, Transform};

/// Circle tool.
/// Use [CircleBuilder][CircleBuilder] as an entrypoint.
///
/// ## Example
/// ```rust
/// use staticmap::tools::CircleBuilder;
///
/// let circle = CircleBuilder::default()
///     .lat_coordinate(4.5)
///     .lon_coordinate(44.2)
///     .radius(4.)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Circle {
    lat_coordinate: f64,
    lon_coordinate: f64,
    color: Color,
    radius: f64,
    radius_in_meters: bool,
    stroke_width: Option<f32>,
}

/// Builder for [Circle][Circle].
#[derive(Debug, Clone)]
pub struct CircleBuilder {
    lat_coordinate: Option<f64>,
    lon_coordinate: Option<f64>,
    color: Color,
    radius: f64,
    radius_in_meters: bool,
    stroke_width: Option<f32>,
}

impl Default for CircleBuilder {
    fn default() -> Self {
        Self {
            lat_coordinate: None,
            lon_coordinate: None,
            color: Color::default(),
            radius: 1.,
            radius_in_meters: false,
            stroke_width: None,
        }
    }
}

impl CircleBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// **Required**.
    /// The center of the circle as a latitude coordinate.
    pub fn lat_coordinate(mut self, coordinate: f64) -> Self {
        self.lat_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// The center of the circle as a longitude coordinate.
    pub fn lon_coordinate(mut self, coordinate: f64) -> Self {
        self.lon_coordinate = Some(coordinate);
        self
    }

    /// Use [Color][Color] to generate a color instance.
    /// Default is a black color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Circle radius in pixels.
    /// Default is 1.0.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius as f64;
        self.radius_in_meters = false;
        self
    }

    /// Circle radius in meters.
    pub fn radius_in_meters(mut self, radius: f64) -> Self {
        self.radius = radius;
        self.radius_in_meters = true;
        self
    }

    /// Draw a filled circle (the default).
    pub fn filled(mut self) -> Self {
        self.stroke_width = None;
        self
    }

    /// Draw an open circle.
    /// Stroke `width` is in pixels, and must be >= 0.0.
    /// When set to 0, a hairline stroking will be used.
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Build the tool, consuming the builder.
    /// Returns an error if the builder is missing required fields.
    pub fn build(self) -> Result<Circle> {
        Ok(Circle {
            lat_coordinate: self
                .lat_coordinate
                .ok_or(Error::BuildError("Latitude coordinate not supplied."))?,
            lon_coordinate: self
                .lon_coordinate
                .ok_or(Error::BuildError("Longitude coordinate not supplied."))?,
            color: self.color,
            radius: self.radius,
            radius_in_meters: self.radius_in_meters,
            stroke_width: self.stroke_width,
        })
    }
}

impl Circle {
    fn radius_px(&self, zoom: u8) -> f64 {
        if self.radius_in_meters {
            m_to_px(self.radius, self.lat_coordinate, zoom)
        } else {
            self.radius
        }
    }
}

impl Tool for Circle {
    fn extent(&self, zoom: u8, tile_size: f64) -> (f64, f64, f64, f64) {
        let radius: f64 = self.radius_px(zoom);

        let x = lon_to_x(self.lon_coordinate, zoom);
        let y = lat_to_y(self.lat_coordinate, zoom);

        let lon_min = x_to_lon(x - radius / tile_size, zoom);
        let lat_min = y_to_lat(y + radius / tile_size, zoom);
        let lon_max = x_to_lon(x + radius / tile_size, zoom);
        let lat_max = y_to_lat(y - radius / tile_size, zoom);

        (lon_min, lat_min, lon_max, lat_max)
    }

    fn draw(&self, bounds: &Bounds, mut pixmap: PixmapMut) {
        let mut path_builder = PathBuilder::new();

        let x = bounds.x_to_px(lon_to_x(self.lon_coordinate, bounds.zoom));
        let y = bounds.y_to_px(lat_to_y(self.lat_coordinate, bounds.zoom));

        path_builder.push_circle(x as f32, y as f32, self.radius_px(bounds.zoom) as f32);

        if let Some(path) = path_builder.finish() {
            if let Some(width) = self.stroke_width {
                pixmap.stroke_path(
                    &path,
                    &self.color.0,
                    &Stroke {
                        width,
                        ..Default::default()
                    },
                    Transform::default(),
                    None,
                );
            } else {
                pixmap.fill_path(
                    &path,
                    &self.color.0,
                    FillRule::default(),
                    Transform::default(),
                    None,
                );
            }
        }
    }
}
