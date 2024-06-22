use crate::bounds::Bounds;
use tiny_skia::{Paint, PixmapMut, Shader};

mod circle;
mod icon;
mod line;
mod rect;
pub use circle::{Circle, CircleBuilder};
pub use icon::{Icon, IconBuilder};
pub use line::{Line, LineBuilder};
pub use rect::{Rect, RectBuilder};

#[derive(Debug, Clone, Default)]
/// Path color.
///
/// ## Example
///
/// ```rust
/// use staticmap::tools::Color;
///
/// let solid_red = Color::new(true, 255, 0, 0, 255);
/// let semitransparent_blue = Color::new(true, 0, 255, 0, 125);
/// ```
pub struct Color(Paint<'static>);

impl Color {
    /// Creates a new [Color][Color] instance based on RGBA values.
    pub fn new(anti_alias: bool, r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(Paint {
            shader: Shader::SolidColor(tiny_skia::Color::from_rgba8(r, g, b, a)),
            anti_alias,
            ..Default::default()
        })
    }
}

/// Trait implemented by types which can be drawn to a map.
pub trait Tool {
    /// Coordinates forming the extent of the object.
    fn extent(&self, zoom: u8, tile_size: f64) -> (f64, f64, f64, f64);
    /// Draw the object to the pixmap using a PathBuilder.
    fn draw(&self, bounds: &Bounds, pixmap: PixmapMut);
}
