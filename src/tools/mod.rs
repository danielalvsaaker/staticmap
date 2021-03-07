use crate::StaticMap;
use tiny_skia::{Paint, Pixmap, Shader};

mod circle;
mod icon;
mod line;
pub use circle::{Circle, CircleBuilder};
pub use icon::{Icon, IconBuilder};
pub use line::{Line, LineBuilder};

#[derive(Clone, Default)]
/// Path color.
///
/// ## Example
///
/// ```rust
/// use staticmap::Color;
///
/// let solid_red = Color::new(true, 255, 0, 0, 255);
/// let semitransparent_blue = Color::new(true, 0, 255, 0, 125);
/// ```
pub struct Color(Paint<'static>);

impl Color {
    pub fn new(anti_alias: bool, r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(Paint {
            shader: Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a)),
            anti_alias,
            ..Default::default()
        })
    }
}

#[doc(hidden)]
/// Generic trait implemented by types which can be drawn to a map.
pub trait Tool {
    /// Coordinates forming the extent of the object.
    fn extent(&self) -> (f64, f64, f64, f64);
    /// Draws the object to the pixmap using a PathBuilder.
    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap);
}

/// Specific trait for markers.
pub trait Marker: Tool {
    fn lon_coordinate(&self) -> f64;
    fn lat_coordinate(&self) -> f64;
}
