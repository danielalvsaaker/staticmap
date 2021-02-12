use staticmap::{Color, LineBuilder, StaticMapBuilder, StaticMapError};

fn main() -> Result<(), StaticMapError> {
    let mut map = StaticMapBuilder::default()
        .width(400)
        .height(400)
        .url_template("https://c.tile.openstreetmap.org/%z/%x/%y.png")
        .build()
        .unwrap();

    let lat: &[f64] = &[52.5, 48.9];
    let lon: Vec<f64> = vec![13.4, 2.3];

    let red = Color::new(true, 255, 0, 0, 255);
    let white = Color::new(true, 255, 255, 255, 255);

    let line = LineBuilder::default()
        .lat_coordinates(lat)
        .lon_coordinates(lon.clone())
        .width(6.)
        .simplify(true)
        .color(red)
        .build()
        .unwrap();

    let underline = LineBuilder::default()
        .lat_coordinates(lat)
        .lon_coordinates(lon)
        .width(15.)
        .simplify(true)
        .color(white)
        .build()
        .unwrap();

    map.add_line(underline);
    map.add_line(line);

    map.save_png("test3.png")?;

    Ok(())
}
