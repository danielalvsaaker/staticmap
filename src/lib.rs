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
//!     StaticMapBuilder, StaticMapError,
//! };
//!
//! fn main() -> Result<(), StaticMapError> {
//!     let mut map = StaticMapBuilder::default()
//!         .width(300)
//!         .height(400)
//!         .padding((10, 0))
//!         .build()
//!         .unwrap();
//!
//!     // Coordinates can be either vec or slice
//!     let lat: &[f64] = &[52.5, 48.9];
//!     let lon: Vec<f64> = vec![13.4, 2.3];
//!
//!     let red = Color::new(true, 255, 0, 0, 255);
//!
//!     let line = LineBuilder::default()
//!         .lat_coordinates(lat)
//!         .lon_coordinates(lon)
//!         .width(3.)
//!         .simplify(true)
//!         .color(red)
//!         .build();
//!
//!     map.add_line(line);
//!     map.save_png("line.png")?;
//!
//!     Ok(())
//! }
//! ```

mod error;
mod map;

/// Line and marker tools.
pub mod tools;

pub use error::StaticMapError;
pub use map::{StaticMap, StaticMapBuilder};

use std::f64::consts::PI;

type Result<T> = std::result::Result<T, StaticMapError>;

fn lon_to_x(mut lon: f64, zoom: u8) -> f64 {
    if !(-180_f64..180_f64).contains(&lon) {
        lon = (lon + 180_f64) % 360_f64 - 180_f64;
    }

    ((lon + 180_f64) / 360_f64) * 2_f64.powi(zoom.into())
}

fn lat_to_y(mut lat: f64, zoom: u8) -> f64 {
    if !(-90_f64..90_f64).contains(&lat) {
        lat = (lat + 90_f64) % 180_f64 - 90_f64;
    }

    (1_f64 - ((lat * PI / 180_f64).tan() + 1_f64 / (lat * PI / 180_f64).cos()).ln() / PI) / 2_f64
        * 2_f64.powi(zoom.into())
}

fn y_to_lat(y: f64, zoom: u8) -> f64 {
    (PI * (1_f64 - 2_f64 * y / 2_f64.powi(zoom.into())))
        .sinh()
        .atan()
        / PI
        * 180_f64
}

fn x_to_lon(x: f64, zoom: u8) -> f64 {
    x / 2_f64.powi(zoom.into()) * 360_f64 - 180_f64
}

fn simplify(points: Vec<(f64, f64)>, tolerance: u8) -> Vec<(f64, f64)> {
    if points.len() < 2 {
        return points;
    }

    let (new_coordinates, points) = points.split_at(1);
    let mut new_coordinates = new_coordinates.to_vec();

    for point in points {
        let a = new_coordinates.last().unwrap();
        let x = ((a.0 - point.0).powi(2) + (a.1 - point.1).powi(2)).sqrt();

        if x > tolerance as f64 {
            new_coordinates.push(*point)
        }
    }

    new_coordinates.push(*points.last().unwrap());

    new_coordinates
}
