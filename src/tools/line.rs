use crate::{lat_to_y, lon_to_x, simplify, tools::Tool, StaticMap};
use derive_builder::Builder;
use tiny_skia::{LineCap, Paint, PathBuilder, Pixmap, Stroke, Transform};

#[derive(Builder)]
pub struct Line {
    #[builder(setter(into))]
    pub(crate) lat_coordinates: Vec<f64>,
    #[builder(setter(into))]
    pub(crate) lon_coordinates: Vec<f64>,
    #[builder(default)]
    pub(crate) color: Paint<'static>,
    #[builder(default = "1.0")]
    pub(crate) width: f32,
    #[builder(default)]
    pub(crate) simplify: bool,
}

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
                    map.x_to_px(lon_to_x(*x, map.zoom.unwrap())) * 2f64,
                    map.y_to_px(lat_to_y(*y, map.zoom.unwrap())) * 2f64,
                )
            })
            .collect();

        if self.simplify {
            points = simplify(points);
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
            &self.color,
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
