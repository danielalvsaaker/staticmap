use raqote::*;
use core::f64::consts::PI;
use image::{RgbaImage, DynamicImage, ImageFormat, ColorType};
use image::imageops::{resize, overlay, FilterType};
use rayon::prelude::*;
use attohttpc::Session;

pub use raqote::SolidSource as Color;
pub use image::ImageFormat::Png as Png;

mod utils;
use utils::{into_rgba, replace};


pub struct Line {
   pub coordinates: Vec<(f64, f64)>,
   pub color: Color,
   pub width: f32,
   pub simplify: bool,
}

impl Line {
    fn extent(&self) -> (f64, f64, f64, f64) {
        let coordinates = &self.coordinates;
        let (lon_min, lat_min, lon_max, lat_max) = (
                coordinates.iter().map(|x| x.0).fold(0./0., f64::min),
                coordinates.iter().map(|x| x.1).fold(0./0., f64::min),
                coordinates.iter().map(|x| x.0).fold(0./0., f64::max),
                coordinates.iter().map(|x| x.1).fold(0./0., f64::max)
            );

        (lon_min, lat_min, lon_max, lat_max)
    }
}

pub struct StaticMap {
    pub width: u32,
    pub height: u32,
    pub padding: (u32, u32),
    pub x_center: f64,
    pub y_center: f64,
    pub url_template: String,
    pub tile_size: u32,
    pub lines: Vec<Line>,
    pub zoom: i32,
}

impl StaticMap {
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn render(&mut self) -> RgbaImage {
        if self.lines.is_empty() {
            panic!("Cannot render empty map, add a line first.");
        }

        self.zoom = self.calculate_zoom();

        let extent = self.determine_extent();

        let mut lon_center = (extent.0 + extent.2) / 2.0;
        let mut lat_center = (extent.1 + extent.3) / 2.0;
        self.x_center = lon_to_x(&mut lon_center, self.zoom);
        self.y_center = lat_to_y(&mut lat_center, self.zoom);

        let mut image = RgbaImage::new(self.width, self.height);
        image = self.draw_base_layer(image);

        // Define a png encoder, which encodes the plot to the png format and writes to the
        // plot-vector
        
        let mut plot: Vec<u8> = Vec::new();

        image::png::PngEncoder::new(&mut plot).encode(&into_rgba(self.draw_features()), self.width * 2, self.height * 2, ColorType::Rgba8).unwrap();

        // Loads plot from slice to DynamicImage, and resizes to size of StaticMap
        let resized = resize(&image::load_from_memory_with_format(&plot, ImageFormat::Png).unwrap(), self.width, self.height, FilterType::CatmullRom);

        overlay(&mut image, &resized, 0, 0);

        image
    }

    fn determine_extent(&self) -> (f64, f64, f64, f64) {
        let mut extent: Vec<(f64, f64, f64, f64)> = Vec::new();
        for line in &self.lines {
            extent.push(line.extent());
        }

        let lon_min: f64 = extent.iter().map(|x| x.0).fold(0./0., f64::min);
        let lat_min: f64 = extent.iter().map(|x| x.1).fold(0./0., f64::min);
        let lon_max: f64 = extent.iter().map(|x| x.2).fold(0./0., f64::max);
        let lat_max: f64 = extent.iter().map(|x| x.3).fold(0./0., f64::max);

        (lon_min, lat_min, lon_max, lat_max)
    }

    fn calculate_zoom(&self) -> i32 {
        let mut zoom: i32 = 0;

        for z in (-1..=17).rev() {
            let mut extent = self.determine_extent();

            let width: f64 = (lon_to_x(&mut extent.2, z) - lon_to_x(&mut extent.0, z)) * self.tile_size as f64;
            if width > (self.width - self.padding.0 * 2).into() {
                continue
            }

            let height = (lat_to_y(&mut extent.1, z) - lat_to_y(&mut extent.3, z)) * self.tile_size as f64;
            if height > (self.height - self.padding.1 * 2) as f64 {
                continue
            }

            zoom = z as i32;
            break;
        };
        zoom
    }

    fn x_to_px(&self, x: f64) -> f64 {
        let px = (x - self.x_center) * self.tile_size as f64 + self.width as f64 / 2f64;
        px.round()
    }

    fn y_to_px(&self, y: f64) -> f64 {
        let px = (y - self.y_center) * self.tile_size as f64 + self.height as f64 / 2f64;
        px.round()
    }

    fn draw_base_layer(&self, mut image: RgbaImage) -> RgbaImage {
        let x_min = (self.x_center - (0.5 * self.width as f64 / self.tile_size as f64)).floor() as i32;
        let y_min = (self.y_center - (0.5 * self.height as f64 / self.tile_size as f64)).floor() as i32;
        let x_max = (self.x_center + (0.5 * self.width as f64 / self.tile_size as f64)).ceil() as i32;
        let y_max = (self.y_center + (0.5 * self.height as f64 / self.tile_size as f64)).ceil() as i32;


        let mut tiles: Vec<(i32, i32, String)> = Vec::new();
        for x in x_min..x_max {
            for y in y_min..y_max{
                let max_tile: i32 = 2i32.pow(self.zoom as u32);
                let tile_x: i32 = (x + max_tile) % max_tile;
                let tile_y: i32 = (y + max_tile) % max_tile;

                let url = self.url_template.replace("%z", &self.zoom.to_string()).replace("%x", &tile_x.to_string()).replace("%y", &tile_y.to_string());
                tiles.push((x, y, url));
            }
        }
        
        let client = Session::new();
        let tile_images: Vec<DynamicImage> = tiles.par_iter().map(|x| {
                let bytes = client.get(&x.2).send().unwrap().bytes().unwrap();
                image::load_from_memory_with_format(bytes.as_slice(), ImageFormat::Png).unwrap()
                }
            ).collect();

        for (tile, tile_image) in tiles.iter().zip(tile_images.iter()) {
            let (x, y) = (tile.0, tile.1);
            let (x_px, y_px) = (self.x_to_px(x.into()), self.y_to_px(y.into()));

            replace(&mut image, tile_image, x_px as i32, y_px as i32); 
        }

        image
    }

    fn draw_features(&self) -> DrawTarget {

        let mut draw_target = DrawTarget::new((self.width * 2) as i32, (self.height * 2) as i32);

        for line in self.lines.iter() {
            let mut path_builder = PathBuilder::new();
            let mut points: Vec<(f64, f64)> = line.coordinates.to_owned().iter_mut().map(|(x, y)| (self.x_to_px(lon_to_x(x, self.zoom)) * 2f64, self.y_to_px(lat_to_y(y, self.zoom)) * 2f64)).collect();

            if line.simplify {
                points = simplify(points);
            }
            for (index, point) in points.iter().enumerate() {
                let (x, y) = (point.0 as f32, point.1 as f32);
                match index {
                   0 => path_builder.move_to(x, y),
                   _ => path_builder.line_to(x, y),
                }
            }

            let path = path_builder.finish();
            draw_target.stroke(&path, &Source::Solid(line.color),
            &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Round,
                width: line.width,
                miter_limit: 10.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new()
            );
        }
        
        draw_target
    }


}
        
        


fn lon_to_x(lon: &mut f64, zoom: i32) -> f64 {
   if !(-180f64 <= *lon && *lon <= 180f64){
       *lon = (*lon + 180f64) % 360f64 - 180f64;
   }

   ((*lon + 180f64) / 360f64) * 2f64.powi(zoom)
}


fn lat_to_y(lat: &mut f64, zoom: i32) -> f64 {
   if !(-90f64 <= *lat && *lat <= 90f64){
       *lat = (*lat + 90f64) % 180f64 - 90f64;
   }
    
   (1f64 - ((*lat * PI / 180f64).tan() + 1f64 / (*lat * PI / 180f64).cos()).ln() / PI) / 2f64 * 2f64.powi(zoom)
}


fn y_to_lat(y: f64, zoom: i32) -> f64 {
   (PI * (1f64 - 2f64 * y / 2f64.powi(zoom))).sinh().atan() / PI * 180f64
}


fn x_to_lon(x: f64, zoom: i32) -> f64 {
   x / 2f64.powi(zoom) * 360f64 - 180f64
}


fn simplify(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {

    let mut new_coordinates: Vec<(f64, f64)> = Vec::new();
    let last_element = points.len() - 1usize;

    for (index, point) in points.iter().enumerate() {
        match Some(index) {
            Some(0) => new_coordinates.push(*point),
            _ if index == last_element => new_coordinates.push(*point),
            _ => {
                if ((new_coordinates.last().unwrap().0 - point.0).powi(2) + (new_coordinates.last().unwrap().1 - point.1).powi(2)).sqrt() > 11f64 {
                    new_coordinates.push(*point)
                }
            },
        }
    }
    
    new_coordinates
}
