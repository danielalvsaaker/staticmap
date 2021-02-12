use crate::{
    lat_to_y, lon_to_x,
    tools::{Marker, Tool},
    StaticMap,
};
use derive_builder::Builder;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

#[derive(Builder)]
pub struct Circle {
    #[builder(setter(into))]
    pub(crate) lat_coordinate: f64,
    #[builder(setter(into))]
    pub(crate) lon_coordinate: f64,
    #[builder(default)]
    pub(crate) color: Paint<'static>,
    #[builder(default = "1.0")]
    pub(crate) radius: f32,
}

impl Tool for Circle {
    fn extent(&self) -> (f64, f64, f64, f64) {
        let radius: f64 = self.radius.into();
        (radius, radius, radius, radius)
    }

    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap) {
        let mut path_builder = PathBuilder::new();

        let x = map.x_to_px(lon_to_x(self.lon_coordinate, map.zoom.unwrap())) * 2_f64;
        let y = map.y_to_px(lat_to_y(self.lat_coordinate, map.zoom.unwrap())) * 2_f64;

        path_builder.push_circle(x as f32, y as f32, self.radius);

        path_builder.close();
        let path = path_builder.finish().unwrap();

        pixmap.fill_path(
            &path,
            &self.color,
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
