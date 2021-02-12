use crate::StaticMap;
use tiny_skia::{Paint, Pixmap, Shader};

pub mod circle;
pub mod line;
pub use circle::{Circle, CircleBuilder};
pub use line::{Line, LineBuilder};

pub struct Color {}

impl Color {
    pub fn new(anti_alias: bool, r: u8, g: u8, b: u8, a: u8) -> Paint<'static> {
        Paint {
            shader: Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a)),
            anti_alias,
            ..Default::default()
        }
    }
}

pub trait Tool {
    fn extent(&self) -> (f64, f64, f64, f64);
    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap);
}

pub trait Marker: Tool {
    fn lon_coordinate(&self) -> f64;
    fn lat_coordinate(&self) -> f64;
}
