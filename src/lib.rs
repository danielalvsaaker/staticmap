//! StaticMap is a library for rendering images of tile based maps.
//!
//! StaticMap uses a builder pattern for building maps, lines and markers.
//!
//! To get started, build a map instance using [StaticMapBuilder][StaticMapBuilder]
//! and find the tool builders in the [tools][tools] module.
//!
//! ### Features:
//! - Render a map to a PNG image.
//! - Draw features on a map, such as:
//!     - Lines
//!     - Circles
//!     - PNG icons
//!
//! ## Example
//! ```rust
//! use staticmap::{
//!     tools::{Color, LineBuilder},
//!     StaticMapBuilder, Error,
//! };
//!
//! fn main() -> Result<(), Error> {
//!     let mut map = StaticMapBuilder::default()
//!         .width(300)
//!         .height(400)
//!         .padding((10, 0))
//!         .build()?;
//!
//!     let lat: &[f64] = &[52.5, 48.9];
//!     let lon: Vec<f64> = vec![13.4, 2.3];
//!
//!     let red = Color::new(true, 255, 0, 0, 255);
//!
//!     let line = LineBuilder::default()
//!         .lat_coordinates(lat.to_vec())
//!         .lon_coordinates(lon)
//!         .width(3.)
//!         .simplify(true)
//!         .color(red)
//!         .build()?;
//!
//!     map.add_tool(line);
//!     map.save_png("line.png")?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

mod bounds;
mod error;
mod map;

/// Tools for drawing features onto the map.
pub mod tools;

pub use bounds::Bounds;
pub use error::Error;
pub use map::{StaticMap, StaticMapBuilder};

use std::f64::consts::PI;

type Result<T> = std::result::Result<T, Error>;

/// Longitude to x coordinate.
pub fn lon_to_x(mut lon: f64, zoom: u8) -> f64 {
    if !(-180_f64..180_f64).contains(&lon) {
        lon = (lon + 180_f64) % 360_f64 - 180_f64;
    }

    ((lon + 180_f64) / 360_f64) * 2_f64.powi(zoom.into())
}

/// Latitude to y coordinate.
pub fn lat_to_y(mut lat: f64, zoom: u8) -> f64 {
    if !(-90_f64..90_f64).contains(&lat) {
        lat = (lat + 90_f64) % 180_f64 - 90_f64;
    }

    (1_f64 - ((lat * PI / 180_f64).tan() + 1_f64 / (lat * PI / 180_f64).cos()).ln() / PI) / 2_f64
        * 2_f64.powi(zoom.into())
}

/// X to longitude coordinate.
pub fn x_to_lon(x: f64, zoom: u8) -> f64 {
    x / 2_f64.powi(zoom.into()) * 360_f64 - 180_f64
}

/// Y to latitude coordinate.
pub fn y_to_lat(y: f64, zoom: u8) -> f64 {
    (PI * (1_f64 - 2_f64 * y / 2_f64.powi(zoom.into())))
        .sinh()
        .atan()
        / PI
        * 180_f64
}

fn simplify(points: Vec<(f64, f64)>, tolerance: f64) -> Vec<(f64, f64)> {
    if points.len() < 2 {
        return points;
    }

    let (simplified_points, points) = points.split_at(1);
    let mut simplified_points = simplified_points.to_vec();

    for point in points {
        if let Some(a) = simplified_points.last() {
            let x = ((a.0 - point.0).powi(2) + (a.1 - point.1).powi(2)).sqrt();

            if x > tolerance {
                simplified_points.push(*point)
            }
        }
    }

    let last_point = points
        .last()
        .expect("Internal logic error - 'points' must have at least two points");
    simplified_points.push(*last_point);
    simplified_points
}
