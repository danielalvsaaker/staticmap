//use raqote::*;
use core::f64::consts::PI;
use image::{RgbaImage, Rgba, DynamicImage, SubImage};
use image::imageops::replace;
use rayon::prelude::*;


struct Line {
   coordinates: Vec<(f64, f64)>,
   color: String,
   width: u32,
   simplify: bool,
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

struct StaticMap {
    width: u32,
    height: u32,
    padding: (u32, u32),
    x_center: f64,
    y_center: f64,
    url_template: String,
    tile_size: u32,
    lines: Vec<Line>,
    zoom: i32,
}

impl StaticMap {
    fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    fn render(&mut self, zoom: Option<i32>) -> RgbaImage {
        if self.lines.is_empty() {
            panic!("Cannot render empty map, add a line first.");
        }

        self.zoom = match zoom {
            Some(x) => x,
            None => self.calculate_zoom(),
        };

        let extent = self.determine_extent();

        let mut lon_center = (extent.0 + extent.3) / 2.0;
        let mut lat_center = (extent.1 + extent.3) / 2.0;
        self.x_center = lon_to_x(&mut lon_center, self.zoom);
        self.y_center = lat_to_y(&mut lat_center, self.zoom);

        let mut image = RgbaImage::new(self.width, self.height);
        image = self.draw_base_layer(image);
        image = self.draw_features(image);

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
        for z in (-1..17).rev() {
            let mut extent = self.determine_extent();

            let width = (lon_to_x(&mut extent.2, z) - lon_to_x(&mut extent.0, z)) * self.tile_size as f64;
            if width > (self.width - self.padding.0 * 2) as f64 {
                continue
            }

            let height = (lat_to_y(&mut extent.1, z) - lat_to_y(&mut extent.3, z)) * self.tile_size as f64;
            if height > (self.height - self.padding.1 * 2) as f64 {
                continue
            }

            return z
        }
        0i32
    }

    fn x_to_px(&self, x: f64) -> u32 {
        let px = (x - self.x_center) * self.tile_size as f64 + self.width as f64 / 2f64;
        px.round() as u32
    }

    fn y_to_px(&self, y: f64) -> u32 {
        let px = (y - self.y_center) * self.tile_size as f64 + self.width as f64 / 2f64;
        px.round() as u32
    }

    fn draw_base_layer(&self, mut image: RgbaImage) -> RgbaImage {
        let x_min = (self.x_center - (0.5 * self.width as f64 / self.tile_size as f64)).floor() as u32;
        let y_min = (self.y_center - (0.5 * self.height as f64 / self.tile_size as f64)).floor() as u32;
        let x_max = (self.x_center + (0.5 * self.width as f64 / self.tile_size as f64)).ceil() as u32;
        let y_max = (self.y_center + (0.5 * self.height as f64 / self.tile_size as f64)).ceil() as u32;


        let mut tiles: Vec<(u32, u32, String)> = Vec::new();
        for (x, y) in (x_min..x_max).zip(y_min..y_max) {
            let max_tile: u32 = 2u32.pow(self.zoom as u32);
            let tile_x: u32 = (x + max_tile) % max_tile;
            let tile_y: u32 = (y + max_tile) % max_tile;

            let url = self.url_template.replace("%z", &self.zoom.to_string()).replace("%x", &tile_x.to_string()).replace("%y", &tile_y.to_string());
            tiles.push((x, y, url));
        }

        let tile_images: Vec<DynamicImage> = tiles.par_iter().map(|x| image::load_from_memory(
                &reqwest::blocking::get(&x.2).unwrap().bytes().unwrap())
                .unwrap()).collect();

        for (tile, tile_image) in tiles.iter().zip(tile_images.iter()) {
            let (x, y) = (tile.0, tile.1);
            //tile_image = SubImage::new(tile_image, 0, 0, tile_image.width(), tile_image.height());

            replace(&mut image, tile_image, self.x_to_px(x.into()), self.y_to_px(y.into())); 
        }

        image
    }

    fn draw_features(&self, mut image: RgbaImage) -> RgbaImage {
        let mut image_lines = RrgbaImage::new((self.width * 2), (self.height * 2));
        for line in self.lines.iter() {
            let points: Vec<(f64, f64)> = line.coordinates.iter().map(|(x, y)| (self.x_to_px(lon_to_x(x, self.zoom) * 2), self.y_to_px(lat_to_y(y, self.zoom)) * 2)).collect();

            if line.simplify {
                points = simplify(points);
            }

            for (index, point) in points.iter().enumerate() {
                length = points.len();
                match Some(index) {
                   Some(0) => (),
                   Some(length) => (),
                   _ => {
                       let center = (point.0 - points[index - 1], point.1 - points[index - 1]);
                       let height = (center.0 + center.1).sqrt();
                       let color = Rgb([255u8, 0u8,   0u8]);
                       draw_filled_ellipse(&mut image_lines, center, line.width, height, color);
                   }
                }
            }

        }
        image
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


fn simplify(points: Vec<(f64, f64)>, tolerance: i32) -> Vec<(f64, f64)> {

    let mut new_coordinates: Vec<(f64, f64)> = Vec::new();
    let last_element = points.len() - 1usize;

    for (index, point) in points.iter().enumerate() {
        match index {
            0 => new_coordinates.push(*point),
            _ if index == last_element => new_coordinates.push(*point),
            _ => {
                if ((new_coordinates.last().unwrap().0 - point.0).powi(2) + (new_coordinates.last().unwrap().1 - point.1).powi(2)).sqrt() > tolerance as f64 {
                    new_coordinates.push(*point)
                }
            },
        }
    }
    
    new_coordinates
}

fn main(){
    let points = vec![(4.3, 5.2), (4.4, 5.5), (15.5, 20.5), (900.0, 22.0), (901.0, 23.0), (902.0, 24.0)];
    let test = simplify(points, 11i32);
    println!("{:?}", test);
}
