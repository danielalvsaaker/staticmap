use crate::{
    lat_to_y, lon_to_x, simplify,
    tools::{Color, Tool},
    StaticMap,
};
use derive_builder::Builder;
use tiny_skia::{LineCap, PathBuilder, Pixmap, Stroke, Transform};

#[derive(Builder)]
/// Line object.
/// Created using LineBuilder.
///
/// ## Example
///
/// ```rust
/// use staticmap::LineBuilder;
///
///
/// let line = LineBuilder::default()
///     .lat_coordinates(vec![52.5, 48.9])
///     .lon_coordinates(vec![13.4, 2.3])
///     .build()
///     .unwrap();
/// ```
///
pub struct Line {
    #[builder(setter(into))]
    /// Vector or slice of latitude coordinates.
    pub(crate) lat_coordinates: Vec<f64>,
    #[builder(setter(into))]
    /// Vector or slice of longitude coordinates.
    pub(crate) lon_coordinates: Vec<f64>,
    #[builder(default)]
    /// Use [staticmap::Color][crate::Color] to to generate a color instance.
    pub(crate) color: Color,
    #[builder(default = "1.0")]
    /// Line width.
    pub(crate) width: f32,
    #[builder(default)]
    /// Whether to simplify line drawing. Disabled by default.
    /// Enabling reduces line shakiness by leaving out close points.
    pub(crate) simplify: bool,
    #[builder(default = "5")]
    /// Affects line rendering if simplify is enabled. Default is 5.
    pub(crate) tolerance: u8,
}

#[doc(hidden)]
impl Tool for Line {
    fn extent(&self) -> (f64, f64, f64, f64) {
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

    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap) {
        let mut path_builder = PathBuilder::new();
        let mut points: Vec<(f64, f64)> = self
            .lon_coordinates
            .iter()
            .zip(self.lat_coordinates.iter())
            .map(|(x, y)| {
                (
                    map.x_to_px(lon_to_x(*x, map.zoom.unwrap())),
                    map.y_to_px(lat_to_y(*y, map.zoom.unwrap())),
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

        path_builder.close();
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
