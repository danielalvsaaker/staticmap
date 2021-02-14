use crate::{
    lat_to_y, lon_to_x,
    tools::{Color, Marker, Tool},
    StaticMap,
};
use derive_builder::Builder;
use tiny_skia::{FillRule, PathBuilder, Pixmap, Transform};

#[derive(Builder)]
/// Circle object. Created using CircleBuilder.
pub struct Circle {
    /// Latitude coordinate for center of circle.
    pub(crate) lat_coordinate: f64,
    /// Longitude coordinate for center of circle.
    pub(crate) lon_coordinate: f64,
    #[builder(default)]
    /// Use [staticmap::Color][crate::Color] to generate a color instance.
    pub(crate) color: Color,
    #[builder(default = "1.0")]
    /// Circle radius in pixels.
    pub(crate) radius: f32,
}

#[doc(hidden)]
impl Tool for Circle {
    fn extent(&self) -> (f64, f64, f64, f64) {
        let radius: f64 = self.radius.into();
        (radius, radius, radius, radius)
    }

    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap) {
        let mut path_builder = PathBuilder::new();

        let x = map.x_to_px(lon_to_x(self.lon_coordinate, map.zoom.unwrap()));
        let y = map.y_to_px(lat_to_y(self.lat_coordinate, map.zoom.unwrap()));

        path_builder.push_circle(x as f32, y as f32, self.radius);

        path_builder.close();
        let path = path_builder.finish().unwrap();

        pixmap.fill_path(
            &path,
            &self.color.0,
            FillRule::default(),
            Transform::default(),
            None,
        );
    }
}

impl Marker for Circle {
    fn lon_coordinate(&self) -> f64 {
        self.lon_coordinate
    }
    fn lat_coordinate(&self) -> f64 {
        self.lat_coordinate
    }
}
