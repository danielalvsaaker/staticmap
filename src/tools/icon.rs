use crate::{
    lat_to_y, lon_to_x,
    tools::{Marker, Tool},
    StaticMap,
};
use derive_builder::Builder;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

#[derive(Clone)]
pub struct Image(Pixmap);

impl From<Vec<u8>> for Image {
    fn from(v: Vec<u8>) -> Image {
        Image(Pixmap::decode_png(&v).expect("Invalid png data."))
    }
}

impl From<&[u8]> for Image {
    fn from(v: &[u8]) -> Image {
        Image(Pixmap::decode_png(v).expect("Invalid png data."))
    }
}

impl From<String> for Image {
    fn from(v: String) -> Image {
        Image(Pixmap::load_png(v).expect("Invalid path or file format."))
    }
}

impl From<&str> for Image {
    fn from(v: &str) -> Image {
        Image(Pixmap::load_png(v).expect("Invalid path or file format."))
    }
}

#[derive(Builder)]
/// Icon tool.
/// Use [IconBuilder][IconBuilder] as an entrypoint.
///
/// ## Example
/// ```rust
/// use staticmap::tools::IconBuilder;
///
/// let icon = IconBuilder::default()
///     .lat_coordinate(6.5)
///     .lon_coordinate(50.5)
///     .x_offset(3.4)
///     .y_offset(10.)
///     .image("icon.png")
///     .build()
///     .unwrap();
/// ```
pub struct Icon {
    /// **Required**.
    /// Latitude coordinate for center of icon.
    lat_coordinate: f64,

    /// **Required**.
    /// Longitude coordinate for center of icon.
    lon_coordinate: f64,

    /// **Required**.
    /// X position of the tip of the icon in pixels, relative to the left bottom of the map.
    x_offset: f64,

    /// **Required**.
    /// Y position of the tip of the icon in pixels, relative to the left bottom of the map.
    y_offset: f64,

    #[builder(setter(into))]
    /// **Required**.
    /// Takes either a `String`/`&str` to a path containing an icon,
    /// or a `Vec<u8>`/`&[u8]` containing image data.
    ///
    /// The icon **must** be a 8-bit png image.
    /// Panics if the path or data is invalid.
    image: Image,
}

#[doc(hidden)]
impl Tool for Icon {
    fn extent(&self) -> (f64, f64, f64, f64) {
        let (width, height) = (self.image.0.width() as f64, self.image.0.height() as f64);
        (
            self.x_offset as f64,
            height - self.y_offset as f64,
            width - self.x_offset as f64,
            self.y_offset as f64,
        )
    }

    fn draw(&self, map: &StaticMap, pixmap: &mut Pixmap) {
        let x = map.x_to_px(lon_to_x(self.lon_coordinate, map.zoom.unwrap())) - self.x_offset;
        let y = map.y_to_px(lat_to_y(self.lat_coordinate, map.zoom.unwrap())) - self.y_offset;

        pixmap.draw_pixmap(
            x as i32,
            y as i32,
            self.image.0.as_ref(),
            &PixmapPaint::default(),
            Transform::default(),
            None,
        );
    }
}

impl Marker for Icon {
    fn lon_coordinate(&self) -> f64 {
        self.lon_coordinate
    }
    fn lat_coordinate(&self) -> f64 {
        self.lat_coordinate
    }
}
