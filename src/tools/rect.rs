use crate::{
    bounds::Bounds,
    lat_to_y, lon_to_x,
    tools::{Color, Tool},
    Error, Result,
};
use tiny_skia::{self, PathBuilder, PixmapMut, Stroke, Transform};

/// Rect tool.
/// Use [RectBuilder][RectBuilder] as an entrypoint.
///
/// ## Example
/// ```rust
/// use staticmap::tools::RectBuilder;
///
/// let rect = RectBuilder::default()
///     .north_lat_coordinate(north_lat)
///     .south_lat_coordinate(south_lat)
///     .east_lon_coordinate(east_lon)
///     .west_lon_coordinate(west_lon)
///     .color(color)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Rect {
    north_lat_coordinate: f64,
    south_lat_coordinate: f64,
    east_lon_coordinate: f64,
    west_lon_coordinate: f64,
    color: Color,
    stroke_width: Option<f32>,
}

/// Builder for [Rect][Rect].
#[derive(Debug, Clone, Default)]
pub struct RectBuilder {
    north_lat_coordinate: Option<f64>,
    south_lat_coordinate: Option<f64>,
    east_lon_coordinate: Option<f64>,
    west_lon_coordinate: Option<f64>,
    color: Color,
    stroke_width: Option<f32>,
}

impl RectBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Default::default()
    }

    /// **Required**.
    /// The latitude coordinate of the northern edge of the rectangle.
    pub fn north_lat_coordinate(mut self, coordinate: f64) -> Self {
        self.north_lat_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// The latitude coordinate of the southern edge of the rectangle.
    pub fn south_lat_coordinate(mut self, coordinate: f64) -> Self {
        self.south_lat_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// The longitude coordinate of the eastern edge of the rectangle.
    pub fn east_lon_coordinate(mut self, coordinate: f64) -> Self {
        self.east_lon_coordinate = Some(coordinate);
        self
    }

    /// **Required**.
    /// The longitude coordinate of the western edge of the rectangle.
    pub fn west_lon_coordinate(mut self, coordinate: f64) -> Self {
        self.west_lon_coordinate = Some(coordinate);
        self
    }

    /// Use [Color][Color] to generate a color instance.
    /// Default is a black color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Draw a filled rectangle (the default).
    pub fn filled(mut self) -> Self {
        self.stroke_width = None;
        self
    }

    /// Draw an open rectangle.
    /// Stroke `width` is in pixels, and must be >= 0.0.
    /// When set to 0, a hairline stroking will be used.
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Build the tool, consuming the builder.
    /// Returns an error if the builder is missing required fields.
    pub fn build(self) -> Result<Rect> {
        Ok(Rect {
            north_lat_coordinate: self
                .north_lat_coordinate
                .ok_or(Error::BuildError("North latitude coordinate not supplied."))?,
            south_lat_coordinate: self
                .south_lat_coordinate
                .ok_or(Error::BuildError("South latitude coordinate not supplied."))?,
            east_lon_coordinate: self
                .east_lon_coordinate
                .ok_or(Error::BuildError("East longitude coordinate not supplied."))?,
            west_lon_coordinate: self
                .west_lon_coordinate
                .ok_or(Error::BuildError("West longitude coordinate not supplied."))?,
            color: self.color,
            stroke_width: self.stroke_width,
        })
    }
}

impl Tool for Rect {
    fn extent(&self, _zoom: u8, _tile_size: f64) -> (f64, f64, f64, f64) {
        (
            self.west_lon_coordinate,
            self.south_lat_coordinate,
            self.east_lon_coordinate,
            self.north_lat_coordinate,
        )
    }

    fn draw(&self, bounds: &Bounds, mut pixmap: PixmapMut) {
        let left = bounds.x_to_px(lon_to_x(self.west_lon_coordinate, bounds.zoom));
        let top = bounds.y_to_px(lat_to_y(self.north_lat_coordinate, bounds.zoom));
        let right = bounds.x_to_px(lon_to_x(self.east_lon_coordinate, bounds.zoom));
        let bottom = bounds.y_to_px(lat_to_y(self.south_lat_coordinate, bounds.zoom));

        let rect = tiny_skia::Rect::from_ltrb(left as f32, top as f32, right as f32, bottom as f32);
        if let Some(rect) = rect {
            if let Some(width) = self.stroke_width {
                pixmap.stroke_path(
                    &PathBuilder::from_rect(rect),
                    &self.color.0,
                    &Stroke {
                        width,
                        ..Default::default()
                    },
                    Transform::default(),
                    None,
                );
            } else {
                pixmap.fill_rect(rect, &self.color.0, Transform::default(), None);
            }
        }
    }
}
