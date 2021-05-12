use crate::{
    bounds::Bounds,
    lat_to_y, lon_to_x, simplify,
    tools::{Color, Tool},
    Error, Result,
};
use tiny_skia::{LineCap, PathBuilder, PixmapMut, Stroke, Transform};

/// Line tool.
/// Use [LineBuilder][LineBuilder] as an entrypoint.
/// ## Example
/// ```rust
/// use staticmap::tools::LineBuilder;
///
/// let line = LineBuilder::default()
///     .lat_coordinates(vec![52.5, 48.9])
///     .lon_coordinates(vec![13.4, 2.3])
///     .build()
///     .unwrap();
/// ```
pub struct Line {
    lat_coordinates: Vec<f64>,
    lon_coordinates: Vec<f64>,
    color: Color,
    width: f32,
    simplify: bool,
    tolerance: f64,
}

pub struct LineBuilder {
    lat_coordinates: Option<Vec<f64>>,
    lon_coordinates: Option<Vec<f64>>,
    color: Color,
    width: f32,
    simplify: bool,
    tolerance: f64,
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self {
            lat_coordinates: None,
            lon_coordinates: None,
            color: Color::default(),
            width: 1.,
            simplify: false,
            tolerance: 5.,
        }
    }
}

impl LineBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// **Required**.
    /// Takes a collection of latitude coordinates.
    pub fn lat_coordinates<I>(mut self, coordinates: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let coordinates = coordinates.into_iter().collect();
        self.lat_coordinates = Some(coordinates);
        self
    }

    /// **Required**.
    /// Takes a collection of longitude coordinates.
    pub fn lon_coordinates<I>(mut self, coordinates: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let coordinates = coordinates.into_iter().collect();
        self.lon_coordinates = Some(coordinates);
        self
    }

    /// Use [Color][Color] to to generate a color instance.
    /// Default is a black color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Line width.
    /// Default is 1.0.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Whether to simplify line drawing.
    /// Enabling reduces line shakiness by leaving out close points.
    /// Disabled by default.
    pub fn simplify(mut self, simplify: bool) -> Self {
        self.simplify = simplify;
        self
    }

    /// Affects line rendering if simplify is enabled.
    ///
    ///
    /// Represents the minimum distance in pixels between two points.
    /// Default is 5.0.
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Build the tool, consuming the builder.
    /// Returns an error if the builder is missing required fields.
    pub fn build(self) -> Result<Line> {
        Ok(Line {
            lat_coordinates: self
                .lat_coordinates
                .ok_or(Error::BuildError("Latitude coordinates not supplied."))?,
            lon_coordinates: self
                .lon_coordinates
                .ok_or(Error::BuildError("Longitude coordinates not supplied."))?,
            color: self.color,
            width: self.width,
            simplify: self.simplify,
            tolerance: self.tolerance,
        })
    }
}

impl Tool for Line {
    fn extent(&self, _: u8, _: f64) -> (f64, f64, f64, f64) {
        (
            self.lon_coordinates
                .iter()
                .copied()
                .fold(f64::NAN, f64::min),
            self.lat_coordinates
                .iter()
                .copied()
                .fold(f64::NAN, f64::min),
            self.lon_coordinates
                .iter()
                .copied()
                .fold(f64::NAN, f64::max),
            self.lat_coordinates
                .iter()
                .copied()
                .fold(f64::NAN, f64::max),
        )
    }

    fn draw(&self, bounds: &Bounds, mut pixmap: PixmapMut) {
        let mut path_builder = PathBuilder::new();
        let mut points: Vec<(f64, f64)> = self
            .lon_coordinates
            .iter()
            .zip(self.lat_coordinates.iter())
            .map(|(x, y)| {
                (
                    bounds.x_to_px(lon_to_x(*x, bounds.zoom)),
                    bounds.y_to_px(lat_to_y(*y, bounds.zoom)),
                )
            })
            .collect();

        if self.simplify {
            points = simplify(points, self.tolerance);
        }

        for (index, point) in points.iter().enumerate() {
            let (x, y) = (point.0 as f32, point.1 as f32);
            match index {
                0 => path_builder.move_to(x, y),
                _ => path_builder.line_to(x, y),
            }
        }

        let path = path_builder.finish().unwrap();

        pixmap.stroke_path(
            &path,
            &self.color.0,
            &Stroke {
                width: self.width,
                line_cap: LineCap::Round,
                ..Default::default()
            },
            Transform::default(),
            None,
        );
    }
}
